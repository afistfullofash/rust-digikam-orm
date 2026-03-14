//! # Images
//!
//! Models for interacting with the Images withing a Digikam Database
use diesel::prelude::*;

use tracing::{debug, error};

use crate::db::{DigikamDatabaseError, get_connection};

use crate::models::albums::{AlbumRoots, Albums};
use crate::models::models::{DigikamModel, HasUniqueHash};
use crate::models::tags::{ImageTags, Tag, Tags};

use crate::schema::AlbumRoots::dsl as album_roots_dsl;
use crate::schema::Albums::dsl as albums_dsl;
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
#[derive(Debug, Clone, DigikamModel)]
pub struct Image {
    /// The Sqlite Connection to the digiKam Database
    connection: String,
    /// If the model has been initalized with database data
    initalized: bool,
    /// The image id in the database
    pub id: i32,
    /// The album id of the `Album` the `Image` is in
    pub album: Option<i32>,
    /// The name of the `Image`. This includes the file extension (i.e: foo.jpg)
    pub name: String,
    pub status: i32,
    pub category: i32,
    pub modification_date: Option<String>,
    /// The size of the `Image`
    pub file_size: Option<i32>,
    /// The unique hash of the `Image` in the database
    pub unique_hash: Option<String>,
    pub manual_order: Option<i32>,

    /// The full file system path to the `Image`
    pub full_path: Option<String>,
    /// The Tags which are applied to the `Image`
    pub tags: Vec<Tag>,
}

impl DigikamModel for Image {
    fn get_connection(self) -> Result<SqliteConnection, DigikamDatabaseError> {
        get_connection(self.connection)
    }

    fn new(connection: String) -> Image {
        Image {
            connection: connection,
            initalized: false,

            id: 0,
            album: None,
            name: "".to_string(),
            status: 0,
            category: 0,
            modification_date: None,
            file_size: None,
            unique_hash: None,
            manual_order: None,

            full_path: None,
            tags: Vec::new(),
        }
    }
    /// Get a `Image` given the Images database id
    ///
    /// # Arguments
    /// * `connection` - A Diesel connection to the digikam sqlite database
    /// * `id` - The Images id in the database
    fn find(self, id: i32) -> Option<Image> {
        match self.clone().get_connection() {
            Ok(mut db_connection) => {
                debug!(id = id, "Finding a Image by id");
                match images_dsl::Images
                    .filter(images_dsl::id.eq(id))
                    .select(ImageTable::as_select())
                    .first::<ImageTable>(&mut db_connection)
                {
                    Ok(i) => {
                        debug!(image = ?i, "Found a Image. Finding the Images tags");
                        let image_tags = Tags::get_for_image(&mut db_connection, i.clone());

                        let full_image_path = self.clone().get_path();

                        Some(Image {
                            connection: self.connection,
                            initalized: true,

                            id: i.id,
                            album: i.album,
                            name: i.name,
                            status: i.status,
                            category: i.category,
                            modification_date: i.modification_date,
                            file_size: i.file_size,
                            unique_hash: i.unique_hash,
                            manual_order: i.manual_order,

                            full_path: full_image_path,
                            tags: image_tags,
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
    fn unique_hash(self) -> Option<&str> {
        self.unique_hash
    }
}

impl Image {
    /// Get the full filesystem path for an `Image`
    /// This requires finding the path to the `Image`'s `Album` and `AlbumRoot`
    ///
    /// # Arguments
    /// * `connection` - A Diesel connection to the digikam database
    /// * `name` - The name of the `Image` we are getting the path for
    /// * `album_id` - The database id of the `Album` this `Image` is in
    pub fn get_path(self) -> Option<String> {
        match self.clone().get_connection() {
            Ok(mut db_connection) => {
                let album = match albums_dsl::Albums
                    .filter(albums_dsl::id.eq(self.album))
                    .select(Albums::as_select())
                    .first::<Albums>(&mut db_connection)
                {
                    Ok(album) => Some(album),
                    Err(e) => {
                        debug!(error= ?e, image = ?self.album, "Error getting the Album this Image Refers to");
                        None
                    }
                };

                let root_path = match &album {
                    Some(a) => {
                        match album_roots_dsl::AlbumRoots
                            .filter(album_roots_dsl::id.eq(a.album_root))
                            .select(AlbumRoots::as_select())
                            .first::<AlbumRoots>(&mut db_connection)
                        {
                            Ok(album_root) => Some(album_root),
                            Err(_) => {
                                debug!(album = ?a, "Error getting the AlbumRoot this Album Refers to");
                                None
                            }
                        }
                    }
                    None => None,
                };

                match (album, root_path) {
                    (Some(a), Some(ar)) => match ar.specific_path {
                        Some(specific_path) => {
                            Some(specific_path + &a.relative_path + "/" + &self.name)
                        }
                        None => None,
                    },
                    _ => None,
                }
            }
            Err(_) => None,
        }
    }

    /// Get an `Vec<Image>` from a `Vec<String>` of Tag Names
    ///
    /// # Arguments
    /// * `connection` - A connection to the Digikam Sqlite Database
    /// * `tag_strings` - A `Vec` of `String` of the `Tag.full_name` to search by
    pub fn find_by_tag_strings(self, tag_strings: &Vec<String>) -> Vec<Image> {
        match self.clone().get_connection() {
            Ok(mut db_connection) => tag_strings
                .iter()
                .map(|tag_string| {
                    let tag = Tag::get_by_path(&mut db_connection, tag_string).unwrap();
                    self.clone().find_by_tag(tag)
                })
                .reduce(|first, second| {
                    Image::keep_common(first, second)
                        .into_iter()
                        .filter(|image| image.full_path.is_some())
                        .collect::<Vec<Image>>()
                })
                .unwrap_or(Vec::new()),
            Err(_) => Vec::new(),
        }
    }

    /// Find `Images` for a given `Tag`
    ///
    /// # Arguments
    /// * `connection` - A connection to the Digikam Sqlite Database
    /// * `tag` - The `Tag` which the `Image`'s are tagged by
    pub fn find_by_tag(self, tag: Tag) -> Vec<Image> {
        match self.clone().get_connection() {
            Ok(mut db_connection) => {
                match image_tags_dsl::ImageTags
                    .filter(image_tags_dsl::tagid.eq(tag.id))
                    .select(ImageTags::as_select())
                    .load(&mut db_connection)
                {
                    Ok(tags) => tags
                        .into_iter()
                        .filter_map(|tag| self.clone().find(tag.imageid))
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
