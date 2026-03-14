//! # Images
//!
//! Models for interacting with the Images withing a Digikam Database
use diesel::prelude::*;

use tracing::debug;

use crate::db::{DigikamDatabaseError, get_connection};

use crate::models::albums::Album;
use crate::models::tags::{ImageTags, Tag};
use crate::models::traits::{DigikamModel, HasUniqueHash};

use crate::schema::ImageTags::dsl as image_tags_dsl;
use crate::schema::Images::dsl as images_dsl;

/// A Library internal representation of the Images table for Digikam
#[derive(Queryable, Selectable, Debug, Identifiable, Clone)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::Images)]
#[diesel(belongs_to(Albums, foreign_key = album))]
pub struct ImageTable {
    /// The image id in the database
    #[diesel(select_expression = crate::schema::Images::id.assume_not_null())]
    pub id: i32,
    /// The album id of the `Album` the Image is in
    pub album: Option<i32>,
    /// The name of the `Image`. This includes the file extension (i.e: foo.jpg)
    pub name: String,
    pub status: i32,
    pub category: i32,
    pub modification_date: Option<String>,
    /// The name of the `Image`. This includes the file extension (i.e: foo.jpg)
    pub file_size: Option<i32>,
    /// The unique hash of the `Image` in the database
    pub unique_hash: Option<String>,
    pub manual_order: Option<i32>,
}

/// A representation of a image from the digiKam Database
/// This has the tags added to it as a `Vec` of `Tags`
#[derive(Debug, Clone)]
pub struct Image {
    /// The Sqlite Connection to the digiKam Database
    connection: String,
    internal_data: Option<ImageTable>,
}

impl DigikamModel for Image {
    fn get_connection(&self) -> Result<SqliteConnection, DigikamDatabaseError> {
        get_connection(&self.connection)
    }

    fn new(connection: &str) -> Image {
        Image {
            connection: connection.to_string(),

            internal_data: None,
        }
    }

    /// Get a `Image` given the Images database id
    ///
    /// # Arguments
    /// * `connection` - A Diesel connection to the digikam sqlite database
    /// * `id` - The Images id in the database
    fn find(&self, id: i32) -> Option<Image> {
        match self.get_connection() {
            Ok(mut db_connection) => {
                debug!(id = id, "Finding a Image by id");
                match images_dsl::Images
                    .filter(images_dsl::id.eq(id))
                    .select(ImageTable::as_select())
                    .first::<ImageTable>(&mut db_connection)
                {
                    Ok(image) => {
                        debug!(image = ?image, "Found a Image. Finding the Images tags");
                        Some(Image {
                            connection: self.connection.clone(),
                            internal_data: Some(image.clone()),
                        })
                    }
                    Err(_) => {
                        debug!(id = id, "Could not get the image by id");
                        None
                    }
                }
            }
            Err(_) => None,
        }
    }
}

impl HasUniqueHash for Image {
    fn unique_hash(&self) -> Option<&str> {
        self.internal_data
            .as_ref()
            .and_then(|image| image.unique_hash.as_deref())
    }
}

impl Image {
    /// Create a new `Image` model bound to a digiKam sqlite connection string.
    pub fn new(connection: &str) -> Image {
        <Image as DigikamModel>::new(connection)
    }

    /// Find an `Image` by its digiKam database id.
    pub fn find_by_id(&self, id: i32) -> Option<Image> {
        <Image as DigikamModel>::find(self, id)
    }

    /// The database id of the image
    pub fn id(&self) -> i32 {
        match &self.internal_data {
            Some(image) => image.id,
            None => 0,
        }
    }

    /// The name of the image
    pub fn name(&self) -> String {
        match &self.internal_data {
            Some(image) => image.name.clone(),
            None => "".to_string(),
        }
    }

    pub fn tags(&self) -> Vec<Tag> {
        Tag::new(&self.connection.clone()).find_by_image(self.clone())
    }

    /// The `Album` the `Image` is in
    pub fn album(&self) -> Option<Album> {
        match &self.internal_data {
            Some(image) => match image.album {
                Some(album_id) => Album::new(&self.connection.clone()).find(album_id),
                None => None,
            },
            None => None,
        }
    }

    /// Get the full filesystem path for an `Image`
    /// This requires finding the path to the `Image`'s `Album` and `AlbumRoot`
    ///
    /// # Arguments
    /// * `connection` - A Diesel connection to the digikam database
    /// * `name` - The name of the `Image` we are getting the path for
    /// * `album_id` - The database id of the `Album` this `Image` is in
    pub fn path(&self) -> Option<String> {
        match self.album() {
            Some(album) => album.path().map(|path| path + "/" + &self.name()),
            None => None,
        }
    }

    /// Get an `Vec<Image>` from a `Vec<String>` of Tag Names
    ///
    /// # Arguments
    /// * `connection` - A connection to the Digikam Sqlite Database
    /// * `tag_strings` - A `Vec` of `String` of the `Tag.full_name` to search by
    pub fn find_by_tag_strings(&self, tag_strings: &[String]) -> Vec<Image> {
        tag_strings
            .iter()
            .map(|tag_string| {
                Tag::new(&self.connection.clone())
                    .find_by_path(tag_string)
                    .map_or_else(Vec::new, |tag| self.find_by_tag(tag))
            })
            .reduce(|first, second| {
                Image::keep_common(first, second)
                    .into_iter()
                    .filter(|image| image.path().is_some())
                    .collect::<Vec<Image>>()
            })
            .unwrap_or_default()
    }

    /// Find `Images` for a given `Tag`
    ///
    /// # Arguments
    /// * `connection` - A connection to the Digikam Sqlite Database
    /// * `tag` - The `Tag` which the `Image`'s are tagged by
    pub fn find_by_tag(&self, tag: Tag) -> Vec<Image> {
        match self.get_connection() {
            Ok(mut db_connection) => {
                let Some(tag_id) = tag.id() else {
                    debug!(tag = ?&tag, "Could not find images for tag without an id");
                    return Vec::new();
                };

                match image_tags_dsl::ImageTags
                    .filter(image_tags_dsl::tagid.eq(tag_id))
                    .select(ImageTags::as_select())
                    .load(&mut db_connection)
                {
                    Ok(tags) => tags
                        .into_iter()
                        .filter_map(|tag| self.find(tag.imageid))
                        .collect::<Vec<Image>>(),
                    Err(_) => {
                        debug!(tag = ?&tag, "Could not find image_tags for tag");
                        Vec::new()
                    }
                }
            }
            Err(_) => Vec::new(),
        }
    }
}
