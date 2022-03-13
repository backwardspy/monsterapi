use std::io::stdout;

use diesel::prelude::*;
use diesel_migrations::embed_migrations;

use crate::Result;

use super::{
    models::{NewCategory, NewProduct},
    schema,
};

embed_migrations!();

pub struct AccessLayer {
    conn: SqliteConnection,
}

impl AccessLayer {
    fn run_migrations(&self) -> Result<()> {
        embedded_migrations::run_with_output(&self.conn, &mut stdout())?;
        Ok(())
    }

    pub fn new(dsn: &str) -> Result<Self> {
        let dal = Self {
            conn: SqliteConnection::establish(dsn)?,
        };
        dal.run_migrations()?;
        Ok(dal)
    }

    pub fn insert_category(&self, category: NewCategory) -> Result<()> {
        diesel::replace_into(schema::category::table)
            .values(&category)
            .execute(&self.conn)?;
        Ok(())
    }

    pub fn insert_product(&self, product: NewProduct) -> Result<()> {
        diesel::replace_into(schema::product::table)
            .values(&product)
            .execute(&self.conn)?;
        Ok(())
    }
}
