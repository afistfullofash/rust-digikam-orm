//! Rust Digikam ORM
//!
//! This is a library which implements a basic orm for querying the DigiKam Sqlite Database
//! It is currently subject to extensive change as the api get's sorted out
mod db;
mod models;
mod schema;

pub use models::albums::Albums;
pub use models::images::Image;
pub use models::tags::Tag;
