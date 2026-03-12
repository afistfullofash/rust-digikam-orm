use diesel::prelude::*;
use std::collections::HashSet;
use tracing::debug;

use crate::models::albums::{AlbumRoots, Albums};
use crate::models::tags::{ImageTags, Tag, Tags};

use crate::schema::AlbumRoots::dsl as album_roots_dsl;
use crate::schema::Albums::dsl as albums_dsl;
use crate::schema::ImageTags::dsl as image_tags_dsl;
use crate::schema::Images::dsl as images_dsl;

#[derive(Queryable, Selectable, Debug, Identifiable, Clone)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::Images)]
#[diesel(belongs_to(Albums, foreign_key = album))]
pub struct ImageTable {
    #[diesel(select_expression = crate::schema::Images::id.assume_not_null())]
    pub id: i32,
    pub album: Option<i32>,
    pub name: String,
    pub status: i32,
    pub category: i32,
    pub modification_date: Option<String>,
    pub file_size: Option<i32>,
    pub unique_hash: Option<String>,
    pub manual_order: Option<i32>,
}

/// A representation of a image from the digiKam Database
/// This has the tags added to it as a Vector of Tags
#[derive(Debug, Clone)]
pub struct Image {
    pub id: i32,
    pub album: Option<i32>,
    pub name: String,
    pub status: i32,
    pub category: i32,
    pub modification_date: Option<String>,
    pub file_size: Option<i32>,
    pub unique_hash: Option<String>,
    pub manual_order: Option<i32>,

    pub full_path: Option<String>,
    pub tags: Vec<Tag>,
}

impl Image {
    pub fn pretty_print(self) {
        println!("Image:");
        println!("  id: {}", self.id);
        println!("  name: {}", self.name);
        match self.full_path {
            Some(path) => println!("  path: {}", path),
            None => println!("  path: No path to image"),
        }

        for tag in self.tags {
            tag.pretty_print();
        }
    }

    pub fn get(connection: &mut SqliteConnection, id: i32) -> Option<Image> {
        debug!(id = id, "getting image by id");
        match images_dsl::Images
            .filter(images_dsl::id.eq(id))
            .select(ImageTable::as_select())
            .first::<ImageTable>(connection)
        {
            Ok(i) => {
                debug!(image = ?i, "Got a image");
                let image_tags = Tags::get_for_image(connection, i.clone());
                let full_image_path = match i.album {
                    Some(album_id) => Image::get_path(connection, &i.name, &album_id),
                    None => None,
                };

                Some(Image {
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

    pub fn get_path(
        connection: &mut SqliteConnection,
        name: &String,
        album_id: &i32,
    ) -> Option<String> {
        let album = match albums_dsl::Albums
            .filter(albums_dsl::id.eq(album_id))
            .select(Albums::as_select())
            .first::<Albums>(connection)
        {
            Ok(album) => Some(album),
            Err(e) => {
                debug!(error= ?e, image = ?album_id, "Error getting the Album this Image Refers to");
                None
            }
        };

        let root_path = match &album {
            Some(a) => {
                match album_roots_dsl::AlbumRoots
                    .filter(album_roots_dsl::id.eq(a.album_root))
                    .select(AlbumRoots::as_select())
                    .first::<AlbumRoots>(connection)
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
                Some(specific_path) => Some(specific_path + &a.relative_path + "/" + name),
                None => None,
            },
            _ => None,
        }
    }
}

pub struct Images {}

impl Images {
    /// Return image ids + resolved filesystem paths for images that match
    /// ALL of the provided tag paths.
    pub fn get_by_tag(connection: &mut SqliteConnection, tag: Tag) -> Vec<Image> {
        match image_tags_dsl::ImageTags
            .filter(image_tags_dsl::tagid.eq(tag.id))
            .select(ImageTags::as_select())
            .load(connection)
        {
            Ok(tags) => tags
                .into_iter()
                .filter_map(|tag| Image::get(connection, tag.imageid))
                .collect::<Vec<Image>>(),
            Err(_) => {
                debug!(tag = ?&tag, "Could not find image_tags for tag");
                Vec::new()
            }
        }
    }

    /// Get a Image by a list of tags in string form
    /// i.e vec!["/Size/wallpaper"]
    pub fn get_by_tag_strings(
        connection: &mut SqliteConnection,
        tag_strings: &Vec<String>,
    ) -> Vec<Image> {
        tag_strings
            .iter()
            .map(|tag_string| {
                let tag = Tag::get_by_path(connection, tag_string).unwrap();
                Images::get_by_tag(connection, tag)
            })
            .reduce(|first, second| {
                Images::keep_common_images(first, second)
                    .into_iter()
                    .filter(|image| image.full_path.is_some())
                    .collect::<Vec<Image>>()
            })
            .unwrap_or(Vec::new())
    }

    /// Given two vectors of Images return a single vector containing only common Images
    /// between the two vectors
    /// In the case either vector is empty we return the vector which is not empty
    fn keep_common_images(left: Vec<Image>, right: Vec<Image>) -> Vec<Image> {
        if left.is_empty() {
            debug!("`left` is empty: Returning the `right` set of images");
            return right;
        }
        if right.is_empty() {
            debug!("`right` is empty: Returning the `left` set of images");
            return left;
        }
        let right_hashes: HashSet<&str> = right
            .iter()
            .filter_map(|img| img.unique_hash.as_deref())
            .collect();

        left.into_iter()
            .filter(|img| {
                img.unique_hash
                    .as_deref()
                    .is_some_and(|hash| right_hashes.contains(hash))
            })
            .collect()
    }
}
