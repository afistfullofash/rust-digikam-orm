//! # Rust Digikam ORM
//!
//! This is a library which implements a basic orm for querying the DigiKam Sqlite Database
//! It is currently subject to extensive change as the api get's sorted out
//!
//! ## Example Usage
//! ```
//! use rust_digikam_orm::Image;
//!
//! let db_path = "~/Pictures/digikam4.db";
//! let tags = Vec::new();
//! Image::new(&db_path).find_by_tag_strings(&tags);
//! ```
mod db;
mod models;
mod schema;

pub use models::albums::Album;
pub use models::images::Image;
pub use models::tags::Tag;

pub use models::traits::DigikamModel;
