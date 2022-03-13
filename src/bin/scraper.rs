use std::path::Path;

use config::Config;

use monsterapi::{database, scraper, Result};

fn main() -> Result<()> {
    let config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .add_source(config::Environment::default())
        .build()?;

    let dal = database::AccessLayer::new(config.get_string("database.dsn")?.as_str())?;

    let output_path = Path::new("./html/");
    let index_path = scraper::download_products_index(output_path)?;
    let categories = scraper::parse_products_index(&index_path)?;

    for category in &categories {
        let new_category = database::models::NewCategory::from(category);
        dal.insert_category(new_category)?;

        for product in &category.products {
            let page_path = scraper::download_product_page(category, product, output_path)?;
            let product = scraper::parse_product_page(&page_path, product)?;
            let new_product = database::models::NewProduct::from(&product);
            dal.insert_product(new_product)?;
        }
    }

    Ok(())
}
