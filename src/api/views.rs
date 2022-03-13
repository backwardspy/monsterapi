use crate::database::models::{Category, Product};
use crate::database::schema::category::dsl::*;
use crate::database::schema::product::dsl::*;
use diesel::prelude::*;
use rocket_contrib::json::Json;

use crate::Result;

use super::DBConn;

#[get("/")]
pub fn index() -> &'static str {
    ""
}

#[get("/categories")]
pub fn category_list(conn: DBConn) -> Result<Json<Vec<Category>>> {
    Ok(Json(category.load::<Category>(&*conn)?))
}

#[get("/categories/<pk>")]
pub fn category_detail(conn: DBConn, pk: String) -> Result<Option<Json<Category>>> {
    Ok(category.find(pk).load::<Category>(&*conn)?.pop().map(Json))
}

#[get("/categories/<pk>/products")]
pub fn category_products_list(conn: DBConn, pk: String) -> Result<Json<Vec<Product>>> {
    Ok(Json(
        product
            .filter(category_slug.eq(pk))
            .load::<Product>(&*conn)?,
    ))
}

#[get("/products")]
pub fn product_list(conn: DBConn) -> Result<Json<Vec<Product>>> {
    Ok(Json(product.load::<Product>(&*conn)?))
}

#[get("/products/<pk>")]
pub fn product_detail(conn: DBConn, pk: String) -> Result<Option<Json<Product>>> {
    Ok(product.find(pk).load::<Product>(&*conn)?.pop().map(Json))
}
