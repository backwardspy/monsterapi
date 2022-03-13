#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

pub mod database;
pub mod scraper;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
