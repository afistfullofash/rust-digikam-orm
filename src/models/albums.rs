use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Identifiable, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::AlbumRoots)]
pub struct AlbumRoots {
    #[diesel(select_expression = crate::schema::AlbumRoots::id.assume_not_null())]
    pub id: i32,
    pub label: Option<String>,
    pub status: i32,
    pub type_: i32,
    pub identifier: Option<String>,
    pub specific_path: Option<String>,
    pub case_sensitivity: Option<i32>,
}

// CREATE TABLE Albums
//                     (id INTEGER PRIMARY KEY,
//                     albumRoot INTEGER NOT NULL,
//                     relativePath TEXT NOT NULL,
//                     date DATE,
//                     caption TEXT,
//                     collection TEXT,
//                     icon INTEGER,
//                     modificationDate DATETIME,
//                     UNIQUE(albumRoot, relativePath));
// CREATE TRIGGER delete_album DELETE ON Albums
//                 BEGIN
//                     DELETE FROM Images
//                     WHERE Images.album = OLD.id;
//                 END;
#[derive(Queryable, Selectable, Debug, Identifiable, Associations, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(AlbumRoots, foreign_key = album_root))]
#[diesel(table_name = crate::schema::Albums)]
pub struct Albums {
    #[diesel(select_expression = crate::schema::Albums::id.assume_not_null())]
    pub id: i32,
    pub album_root: i32,
    pub relative_path: String,
    pub date: Option<String>,
    pub caption: Option<String>,
    pub collection: Option<String>,
    pub icon: Option<i32>,
    pub modification_date: Option<String>,
}
