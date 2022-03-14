#![feature(decl_macro)]

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

pub mod api;
pub mod database;
pub mod scraping;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
