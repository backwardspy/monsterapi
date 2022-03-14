use diesel::prelude::*;
use rocket::response::Debug;
use rocket::serde::json::Json;

use crate::database::models::{Category, Product};
use crate::database::schema::category::dsl::*;
use crate::database::schema::product::dsl::*;

use super::DBConn;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
pub fn index() -> &'static str {
    ""
}

#[get("/categories")]
pub async fn category_list(db: DBConn) -> Result<Json<Vec<Category>>> {
    let results = db.run(move |conn| category.load::<Category>(conn)).await?;
    Ok(Json(results))
}

#[get("/categories/<pk>")]
pub async fn category_detail(db: DBConn, pk: String) -> Result<Option<Json<Category>>> {
    let result = db
        .run(move |conn| category.find(pk).load::<Category>(conn))
        .await?
        .pop()
        .map(Json);
    Ok(result)
}

#[get("/categories/<pk>/products")]
pub async fn category_products_list(db: DBConn, pk: String) -> Result<Json<Vec<Product>>> {
    let results = db
        .run(move |conn| product.filter(category_slug.eq(pk)).load::<Product>(conn))
        .await?;
    Ok(Json(results))
}

#[get("/products")]
pub async fn product_list(db: DBConn) -> Result<Json<Vec<Product>>> {
    let results = db.run(move |conn| product.load::<Product>(conn)).await?;
    Ok(Json(results))
}

#[get("/products/<pk>")]
pub async fn product_detail(db: DBConn, pk: String) -> Result<Option<Json<Product>>> {
    let result = db
        .run(move |conn| product.find(pk).load::<Product>(conn))
        .await?
        .pop()
        .map(Json);
    Ok(result)
}
