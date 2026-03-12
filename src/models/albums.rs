//! # Albums
//!
//! Models for interacting with the Albums within Digikam
use diesel::prelude::*;

/// The Root Settings for an `Album`
#[derive(Queryable, Selectable, Debug, Identifiable, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::AlbumRoots)]
pub struct AlbumRoots {
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

/// Albums within DigiKam
#[derive(Queryable, Selectable, Debug, Identifiable, Associations, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(AlbumRoots, foreign_key = album_root))]
#[diesel(table_name = crate::schema::Albums)]
pub struct Albums {
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
