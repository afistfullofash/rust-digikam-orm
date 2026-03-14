//! # Albums
//!
//! Models for interacting with the Albums within Digikam
use diesel::prelude::*;
use tracing::debug;

use super::traits::DigikamModel;
use crate::db::{DigikamDatabaseError, get_connection};
use crate::schema::AlbumRoots::dsl as album_roots_dsl;
use crate::schema::Albums::dsl as albums_dsl;

/// The Root Settings for an `Album`
#[derive(Queryable, Selectable, Debug, Identifiable, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::AlbumRoots)]
pub struct AlbumRootsTable {
    /// The database id within the Digikam Database
    #[diesel(select_expression = crate::schema::AlbumRoots::id.assume_not_null())]
    pub id: i32,
    pub label: Option<String>,
    pub status: i32,
    pub type_: i32,
    pub identifier: Option<String>,
    pub specific_path: Option<String>,
    pub case_sensitivity: Option<i32>,
}

pub struct AlbumRoot {
    /// The sqlite connection to the digiKam Database
    connection: String,
    /// The `AlbumRoot` as represented in the database
    internal_data: Option<AlbumRootsTable>,
}

impl AlbumRoot {
    /// The path to the `AlbumRoot`
    pub fn path(&self) -> Option<String> {
        match &self.internal_data {
            Some(album_root) => album_root.specific_path.clone(),
            None => None,
        }
    }
}

impl DigikamModel for AlbumRoot {
    /// Get a connection to the digiKam databse
    fn get_connection(&self) -> Result<SqliteConnection, DigikamDatabaseError> {
        get_connection(&self.connection)
    }

    /// Create a new instance of `AlbumRoot` for querying for data
    fn new(connection: &str) -> AlbumRoot {
        AlbumRoot {
            connection: connection.to_string(),
            internal_data: None,
        }
    }

    /// Get a `AlbumRoot` given the AlbumRoots database id
    ///
    /// # Arguments
    /// * `connection` - A Diesel connection to the digikam sqlite database
    /// * `id` - The AlbumRoots id in the database
    fn find(&self, id: i32) -> Option<AlbumRoot> {
        match self.get_connection() {
            Ok(mut db_connection) => {
                match album_roots_dsl::AlbumRoots
                    .filter(album_roots_dsl::id.eq(id))
                    .select(AlbumRootsTable::as_select())
                    .first::<AlbumRootsTable>(&mut db_connection)
                {
                    Ok(album_root) => Some(AlbumRoot {
                        connection: self.connection.clone(),
                        internal_data: Some(album_root),
                    }),
                    Err(_) => {
                        debug!(
                            album = id,
                            "Error getting the AlbumRoot this Album Refers to"
                        );
                        None
                    }
                }
            }
            Err(_) => None,
        }
    }
}

/// Albums within DigiKam
#[derive(Queryable, Selectable, Debug, Identifiable, Associations, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(AlbumRootsTable, foreign_key = album_root))]
#[diesel(table_name = crate::schema::Albums)]
struct AlbumsTable {
    /// The Albums database id
    #[diesel(select_expression = crate::schema::Albums::id.assume_not_null())]
    pub id: i32,
    /// The `AlbumRoot` this Album belongs to
    pub album_root: i32,
    /// The relative path of the `Album` from its `AlbumRoot`
    pub relative_path: String,
    /// The date the Album refers to
    pub date: Option<String>,
    /// The Albums Caption
    pub caption: Option<String>,
    pub collection: Option<String>,
    /// The Albums Icon
    pub icon: Option<i32>,
    pub modification_date: Option<String>,
}

/// An Album inside digiKam
pub struct Album {
    /// The sqlite connection to the digiKam Database
    connection: String,
    /// The album data as show on the database
    internal_data: Option<AlbumsTable>,
}

impl Album {
    /// The `AlbumRoot` this `Album` is a part of
    pub fn album_root(&self) -> Option<AlbumRoot> {
        match &self.internal_data {
            Some(album) => AlbumRoot::new(&self.connection.clone()).find(album.album_root),
            None => None,
        }
    }

    /// The path of this `Album` relative to the `AlbumRoot`
    pub fn relative_path(&self) -> Option<String> {
        self.internal_data
            .as_ref()
            .map(|album| album.relative_path.clone())
    }

    /// The full filesystem path to the album
    pub fn path(&self) -> Option<String> {
        match (self.relative_path(), self.album_root()) {
            (Some(album_relative_path), Some(album_root)) => album_root
                .path()
                .map(|root_path| root_path + &album_relative_path),
            _ => None,
        }
    }
}

impl DigikamModel for Album {
    /// Get a connection to the digiKam Database
    fn get_connection(&self) -> Result<SqliteConnection, DigikamDatabaseError> {
        get_connection(&self.connection)
    }

    /// Create a new instance of `Album` for querying
    fn new(connection: &str) -> Album {
        Album {
            connection: connection.to_string(),
            internal_data: None,
        }
    }

    /// Get a `Album` given the Albums database id
    ///
    /// # Arguments
    /// * `connection` - A Diesel connection to the digikam sqlite database
    /// * `id` - The Albums id in the database
    fn find(&self, id: i32) -> Option<Album> {
        match self.get_connection() {
            Ok(mut db_connection) => {
                match albums_dsl::Albums
                    .filter(albums_dsl::id.eq(id))
                    .select(AlbumsTable::as_select())
                    .first::<AlbumsTable>(&mut db_connection)
                {
                    Ok(album) => Some(Album {
                        connection: self.connection.clone(),
                        internal_data: Some(album),
                    }),
                    Err(e) => {
                        debug!(error= ?e, id = id, "Error finding Album");
                        None
                    }
                }
            }
            Err(_) => None,
        }
    }
}
