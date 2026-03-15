//! # Tags
//!
//! Models for interacting with the Tags within a Digikam Database
use diesel::associations::Identifiable;
use diesel::prelude::*;
use tracing::{debug, error};

use super::traits::DigikamModel;
use crate::db::{get_connection, DigikamDatabaseError};

use crate::models::images::ImageTable;
use crate::schema::ImageTags::dsl as image_tags_dsl;
use crate::schema::Tags::dsl as tags_dsl;

/// Internal Library representation of the Tags table in the Digikam database
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Clone)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(TagsTable, foreign_key = pid))]
#[diesel(table_name = crate::schema::Tags)]
pub struct TagsTable {
    /// The id of the `Tag` in the database
    #[diesel(select_expression = crate::schema::Tags::id.assume_not_null())]
    pub id: i32,
    /// The parent `Tag` of this `Tag`
    pub pid: Option<i32>,
    /// The tags name
    pub name: String,
    /// The icon used to represent the tag
    pub icon: Option<i32>,
    /// The kde icon used to represent the tag
    pub iconkde: Option<String>,
}

/// A Tag inside Digikam
#[derive(Debug, Clone)]
pub struct Tag {
    /// The path to the digiKam Sqlite Database
    database_connection: String,
    /// The tag as it is in the db
    internal_tag: Option<TagsTable>,
    /// The Full Name of the tag expanded for the full path
    /// By default the tag name only includes the name of its leaf in the tag tree
    /// e.g: For the tag /Our Tag {id: 3, pid: Some(2), name: "/Our Tag"}
    ///   - /Root Tag/Parent Tag/Child Tag <-- Full Tag name
    ///     => /Root Tag                   <-- Tag {id: 1, pid: None, name: "/Root Tag"}
    ///     => /Parent Tag                 <-- Tag {id: 2, pid: Some(1), name: "/Parent Tag"}
    ///     => /Child Tag                  <-- Tag {id: 3, pid: Some(2), name: "/Child Tag"}
    full_name: String,
}

impl Tag {
    /// Create a new `Tag` model bound to a digiKam sqlite connection string.
    pub fn new(connection: &str) -> Tag {
        <Tag as DigikamModel>::new(connection)
    }

    /// Find a `Tag` by its digiKam database id.
    pub fn find_by_id(&self, id: i32) -> Option<Tag> {
        <Tag as DigikamModel>::find(self, id)
    }

    /// The id of the tag in the database, if the model was initialized from database data.
    pub fn id(&self) -> Option<i32> {
        self.internal_tag.as_ref().map(|tag| tag.id)
    }

    /// The name of the tag
    pub fn name(&self) -> String {
        match &self.internal_tag {
            Some(tag_details) => tag_details.name.clone(),
            None => "".to_string(),
        }
    }

    /// The full name of the tag
    ///
    /// # Returns
    /// The Full Name of the tag expanded for the full path
    /// By default the tag name only includes the name of its leaf in the tag tree
    /// e.g: For the tag /Our Tag {id: 3, pid: Some(2), name: "/Our Tag"}
    ///   - /Root Tag/Parent Tag/Child Tag <-- Full Tag name
    ///     => /Root Tag                   <-- Tag {id: 1, pid: None, name: "/Root Tag"}
    ///     => /Parent Tag                 <-- Tag {id: 2, pid: Some(1), name: "/Parent Tag"}
    ///     => /Child Tag                  <-- Tag {id: 3, pid: Some(2), name: "/Child Tag"}
    pub fn full_name(&self) -> String {
        self.full_name.clone()
    }

    /// Get the Parent `Tag` for `Self` if it exists
    pub fn parent(&self) -> Option<Tag> {
        match &self.internal_tag {
            Some(tag_details) => match tag_details.pid {
                Some(pid) => self.find(pid),
                None => None,
            },
            None => None,
        }
    }

    /// Get the full tag name as the database does not contain this as a single value
    ///
    /// # Arguments
    /// * `connection` - A Connection to the Digikam Sqlite Database
    fn get_full_name(&self) -> String {
        debug!(tag = ?self, "Getting Full tag name");
        let mut tag_path: Vec<Tag> = Vec::from([self.clone()]);

        let mut current_tag = Some(self.clone());

        while let Some(tag) = current_tag {
            debug!("Checking if this tag has a parent");
            current_tag = tag.parent();
            if let Some(parent_tag) = current_tag.clone() {
                debug!("Got parent tag");
                tag_path.push(parent_tag);
            }

            debug!(tag = ?current_tag, "After checking parenthood the next tag is");
        }
        debug!(tag_path = ?tag_path, "Tags for tag path");
        tag_path.into_iter().rev().fold("".to_string(), |acc, t| {
            let tag_name = acc.clone() + "/" + &t.name();
            debug!(tag_name = tag_name, acc = ?acc, t = ?t, "Forming Name");
            tag_name
        })
    }

    /// Find a `Tag` by searching for it by name
    pub fn find_by_name(&self, name: &str) -> Vec<Tag> {
        match self.get_connection() {
            Ok(mut db_connection) => {
                match tags_dsl::Tags
                    .filter(tags_dsl::name.eq(name))
                    .select(TagsTable::as_select())
                    .load(&mut db_connection)
                {
                    Ok(tags) => tags
                        .into_iter()
                        .map(|tag| (tag, self.database_connection.as_str()).into())
                        .collect::<Vec<Tag>>(),
                    Err(_) => Vec::new(),
                }
            }
            Err(_) => Vec::new(),
        }
    }

    /// Get a `Tag` given a full tag path
    ///
    /// # Arguments
    /// * `connection` - A sqlite connection to the digikam database file
    /// * `path` - The full tag path eg: "/Root Tag/Parent Tag/Our Tag"
    pub fn find_by_path(&self, path: &str) -> Option<Tag> {
        // We have to match from the lowest tag up even though its used top down
        // So we get all the tags matching the last part and then resolve their full names
        // Using that we then match it against the full path which should give us one result
        let segments = path
            .split('/')
            .rev()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        debug!(
            path_segments = ?segments,
            "Getting tag for these path segments"
        );
        debug!(segment = ?segments[0], "The first segment to get");

        let tags = self.find_by_name(segments[0]);

        debug!(tags = ?tags, "The query produced these tags");

        tags.into_iter().fold(None, |acc, tag| {
            debug!(tag = ?tag, path = path, "Checking if this tag is the correct one");
            if tag.full_name == path {
                debug!("Got the correct tag");
                return Some(tag);
            }
            debug!("Did not get the correct tag");
            acc
        })
    }

    /// Get the `Tag`'s for an `Image` given a `ImageTable`
    ///
    /// # Arguments
    /// * `connection` - A sqlite connection to the current database
    /// * `image` - A Image to get the tags for
    pub fn find_for_image(&self, image: ImageTable) -> Vec<Tag> {
        match self.clone().get_connection() {
            Ok(mut db_connection) => {
                let image_tags = image_tags_dsl::ImageTags
                    .filter(image_tags_dsl::imageid.eq(&image.id))
                    .select(ImageTags::as_select())
                    .load(&mut db_connection)
                    .unwrap_or_default();

                image_tags.into_iter().filter_map(|image_tag: ImageTags| -> Option<Tag> {
		    debug!(image_tag = ?image_tag, "Finding the tag for this image_tag");
		    match self.find(image_tag.tagid)
		    {
			Some(tag) => {
			    Some(tag)
			},
			None => {
			    error!(image_tag = ?image_tag, "Error getting tag referenced by image_tag");
			    None
			}
		    }
		}).collect::<Vec<Tag>>()
            }
            Err(_) => Vec::new(),
        }
    }
}

impl DigikamModel for Tag {
    fn get_connection(&self) -> Result<SqliteConnection, DigikamDatabaseError> {
        get_connection(&self.database_connection)
    }

    fn new(connection: &str) -> Tag {
        Tag {
            database_connection: connection.to_string(),
            internal_tag: None,
            full_name: "".to_string(),
        }
    }

    /// Finds a `Tag` given the tags id in the digikam database
    ///
    /// # Arguments
    /// * `connection` - A Sqlite Connection to the Digikam Database
    /// * `id` - The tags id in the database
    fn find(&self, id: i32) -> Option<Tag> {
        match self.get_connection() {
            Ok(mut database_connection) => {
                match tags_dsl::Tags
                    .filter(tags_dsl::id.eq(id))
                    .select(TagsTable::as_select())
                    .first::<TagsTable>(&mut database_connection)
                {
                    Ok(tag) => Some((tag, self.database_connection.as_str()).into()),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
}

impl From<(TagsTable, &str)> for Tag {
    fn from((tagstable, db_connection): (TagsTable, &str)) -> Tag {
        let tag = Tag {
            database_connection: db_connection.to_string(),
            internal_tag: Some(tagstable.clone()),
            full_name: String::new(),
        };

        Tag {
            full_name: tag.get_full_name(),
            ..tag
        }
    }
}

/// A Join Table between `Tag`'s and `Image`'s
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Clone, Copy)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(ImageTable, foreign_key = imageid))]
#[diesel(belongs_to(TagsTable, foreign_key = tagid))]
#[diesel(table_name = crate::schema::ImageTags)]
#[diesel(primary_key(rowid))]
pub struct ImageTags {
    /// The rowid of the `ImageTag` row in the database
    pub rowid: i32,
    /// The database id of the `Image`
    pub imageid: i32,
    /// The database id of the `Tag`
    pub tagid: i32,
}
