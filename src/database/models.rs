use diesel::{Insertable, Queryable};
use serde::Serialize;

use super::schema::{category, product};
use crate::scraping;

#[derive(Queryable, Serialize)]
pub struct Category {
    slug: String,
    name: String,
}

#[derive(Insertable)]
#[table_name = "category"]
pub struct NewCategory {
    slug: String,
    name: String,
}

#[derive(Queryable, Serialize)]
pub struct Product {
    slug: String,
    name: String,
    full_name: String,
    description: String,
    tagline: String,
    flavour_profile: String,
    icon_url: String,
    showcase_image_url: String,
    category_slug: String,
}

#[derive(Insertable)]
#[table_name = "product"]
pub struct NewProduct {
    slug: String,
    name: String,
    full_name: String,
    description: String,
    tagline: String,
    flavour_profile: String,
    icon_url: String,
    showcase_image_url: String,
    category_slug: String,
}

impl From<&scraping::Category> for NewCategory {
    fn from(category: &scraping::Category) -> Self {
        NewCategory {
            slug: category.slug.clone(),
            name: category.name.clone(),
        }
    }
}

impl From<&scraping::Product> for NewProduct {
    fn from(product: &scraping::Product) -> Self {
        NewProduct {
            slug: product.slug.clone(),
            name: product.name.clone(),
            full_name: product.full_name.clone(),
            description: product.description.clone(),
            tagline: product.tagline.clone(),
            flavour_profile: product.flavour_profile.clone(),
            icon_url: product.icon_url.to_string(),
            showcase_image_url: product.showcase_image_url.to_string(),
            category_slug: product.category_slug.clone(),
        }
    }
}
