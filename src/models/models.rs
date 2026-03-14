use diesel::SqliteConnection;

use crate::db::DigikamDatabaseError;

/// Defines the shared features of the digiKam Models
pub trait DigikamModel: Sized {
    fn new(connection: String) -> Self;
    fn get_connection(&self) -> Result<SqliteConnection, DigikamDatabaseError>;
    /// Find a Model in the Databse and return its library representation
    ///
    /// # Arguments
    /// * `id` - The id of the Model in the Database
    fn find(&self, id: i32) -> Option<Self>;

    /// Given two vectors of `Self` return a single vector containing only common instances of `Self`
    ///
    /// # Arguments
    /// * `left` - A `Vec` of `Self` to check comminality for
    /// * `right` - A `Vec` of `Self` to check comminality for
    ///
    /// # Returns
    /// A `Vec<Self>` which contains the common items in both `Vec`'s
    /// - One Empty `Vec` Input: The other `Vec` is returned
    /// - Both Empty: A empty `Vec` is returned
    fn keep_common(left: Vec<Self>, right: Vec<Self>) -> Vec<Self>
    where
        Self: HasUniqueHash,
    {
        if left.is_empty() {
            return right;
        }
        if right.is_empty() {
            return left;
        }

        let right_hashes: std::collections::HashSet<&str> =
            right.iter().filter_map(|item| item.unique_hash()).collect();

        left.into_iter()
            .filter(|item| {
                item.unique_hash()
                    .is_some_and(|hash| right_hashes.contains(hash))
            })
            .collect()
    }
}

pub trait HasUniqueHash {
    fn unique_hash(&self) -> Option<&str>;
}
