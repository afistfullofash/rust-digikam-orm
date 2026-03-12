//! # Tags
//!
//! Models for interacting with the Tags within a Digikam Database
use crate::models::images::ImageTable;
use crate::schema::ImageTags::dsl as image_tags_dsl;
use crate::schema::Tags::dsl as tags_dsl;
use diesel::associations::Identifiable;
use diesel::prelude::*;

use tracing::{debug, error};

/// Internal Library representation of the Tags table in the Digikam database
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Clone)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Tags, foreign_key = pid))]
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

impl TagsTable {
    /// Convert a `TagsTable` Instance into a `Tag`
    ///
    /// # Arguments
    /// * `connection` - A connection to the Digikam Sqlite Database file
    ///
    /// # Returns
    /// A `Tag`
    pub fn to_tag(self, connection: &mut SqliteConnection) -> Tag {
        let full_tag_name = self.get_full_name(connection);
        let tag = Tag {
            id: self.id,
            pid: self.pid,
            name: self.name,
            icon: self.icon,
            iconkde: self.iconkde,
            full_name: full_tag_name,
        };
        debug!(tag = ?tag, "Returning Tag");
        tag
    }

    /// Get the full tag name as the database does not contain this as a single value
    ///
    /// # Arguments
    /// * `connection` - A Connection to the Digikam Sqlite Database
    fn get_full_name(&self, connection: &mut SqliteConnection) -> String {
        debug!(tag = ?self, "Getting Full tag name");
        let mut tag_path: Vec<TagsTable> = Vec::from([self.clone()]);

        let mut current_tag = Some(self.clone());

        while let Some(tag) = current_tag {
            debug!("Checking if this tag has a parent");
            current_tag = match tag.pid {
                Some(pid) => {
                    debug!("Tag has a parent");
                    match tags_dsl::Tags
                        .filter(tags_dsl::id.eq(pid))
                        .select(TagsTable::as_select())
                        .first::<TagsTable>(connection)
                    {
                        Ok(parent_tag) => {
                            debug!("Got parent tag");
                            tag_path.push(parent_tag.clone());
                            Some(parent_tag)
                        }
                        Err(_) => None,
                    }
                }
                None => None,
            };
            debug!(tag = ?current_tag, "After checking parenthood the next tag is");
        }
        debug!(tag_path = ?tag_path, "Tags for tag path");
        tag_path.into_iter().rev().fold("".to_string(), |acc, t| {
            let tag_name = acc.clone() + "/" + &t.name;
            debug!(tag_name = tag_name, acc = ?acc, t = ?t, "Forming Name");
            tag_name
        })
    }
}

/// A Tag inside Digikam
#[derive(Debug, Clone)]
pub struct Tag {
    /// The tag id inside the database
    pub id: i32,
    /// The parent id of the tag
    pub pid: Option<i32>,
    /// The name of the tag
    pub name: String,
    /// The icon the tag uses
    pub icon: Option<i32>,
    /// The kdeicon the tag uses
    pub iconkde: Option<String>,
    /// The Full Name of the tag expanded for the full path
    /// By default the tag name only includes the name of its leaf in the tag tree
    /// e.g: For the tag /Our Tag {id: 3, pid: Some(2), name: "/Our Tag"}
    ///   - /Root Tag/Parent Tag/Child Tag <-- Full Tag name
    ///     => /Root Tag                   <-- Tag {id: 1, pid: None, name: "/Root Tag"}
    ///       => /Parent Tag               <-- Tag {id: 2, pid: Some(1), name: "/Parent Tag"}
    ///         => /Child Tag              <-- Tag {id: 3, pid: Some(2), name: "/Child Tag"}
    pub full_name: String,
}

impl Tag {
    pub fn pretty_print(self) {
        println!("Tag:");
        println!("  Name:  {}", self.full_name);
    }

    /// Get a `Tag` given the tags id in the digikam database
    ///
    /// # Arguments
    /// * `connection` - A Sqlite Connection to the Digikam Database
    /// * `id` - The tags id in the database
    pub fn get(connection: &mut SqliteConnection, id: i32) -> Option<Tag> {
        match tags_dsl::Tags
            .filter(tags_dsl::id.eq(id))
            .select(TagsTable::as_select())
            .first::<TagsTable>(connection)
        {
            Ok(tag) => Some(tag.to_tag(connection)),
            Err(_) => None,
        }
    }

    /// Get a `Tag` given a full tag path
    ///
    /// # Arguments
    /// * `connection` - A sqlite connection to the digikam database file
    /// * `path` - The full tag path eg: "/Root Tag/Parent Tag/Our Tag"
    pub fn get_by_path(connection: &mut SqliteConnection, path: &str) -> Option<Tag> {
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
        let tags = tags_dsl::Tags
            .filter(tags_dsl::name.eq(segments[0]))
            .select(TagsTable::as_select())
            .load(connection)
            .ok();

        debug!(tags = ?tags, "The query produced these tags");

        match tags {
            Some(t) => t
                .into_iter()
                .map(|tt| tt.to_tag(connection))
                .fold(None, |acc, tt| {
                    debug!(tag = ?tt, path = path, "Checking if this tag is the correct one");
                    if tt.full_name == path {
                        debug!("Got the correct tag");
                        return Some(tt);
                    }
                    debug!("Did not get the correct tag");
                    acc
                }),
            None => None,
        }
    }
}

/// A Join Table between `Tag`'s and `Image`'s
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Clone, Copy)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(ImageTable, foreign_key = imageid))]
#[diesel(belongs_to(Tags, foreign_key = tagid))]
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

pub struct Tags {}

impl Tags {
    /// Get the `Tag`'s for an `Image` given a `ImageTable`
    ///
    /// # Arguments
    /// * `connection` - A sqlite connection to the current database
    /// * `image` - A Image to get the tags for
    pub fn get_for_image(connection: &mut SqliteConnection, image: ImageTable) -> Vec<Tag> {
        let image_tags = match image_tags_dsl::ImageTags
            .filter(image_tags_dsl::imageid.eq(&image.id))
            .select(ImageTags::as_select())
            .load(connection)
        {
            Ok(it) => it,
            Err(_) => Vec::new(),
        };

        image_tags.into_iter().filter_map(|image_tag: ImageTags| -> Option<Tag> {
            debug!(image_tag = ?image_tag, "Finding the tag for this image_tag");
            match tags_dsl::Tags.filter(tags_dsl::id.eq(image_tag.tagid))
                .select(TagsTable::as_select())
                .first::<TagsTable>(connection)
            {
                Ok(tag) => {
		    Some(tag.to_tag(connection))
		},
                Err(e) => {
                    error!(error = ?e, image_tag = ?image_tag, "Error getting tag referenced by image_tag");
                    None
                }
            }
	}).collect::<Vec<Tag>>()
    }
}
