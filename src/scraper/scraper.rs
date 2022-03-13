use std::{
    fs,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use headless_chrome::{Browser, FetcherOptions, LaunchOptions};
use rand::random;
use scraper::{ElementRef, Html, Node, Selector};
use url::Url;

use crate::Result;

use super::types::{Category, Product, ProductInfo};

const BASE_URL: &str = "https://www.monsterenergy.com";

macro_rules! select {
    ($el:expr, $sel:expr) => {
        $el.select(&Selector::parse($sel).unwrap())
    };
}

fn mkdir(path: &Path) -> Result<()> {
    if path.exists() {
        if !path.is_dir() {
            return Err(format!("{} is not a directory", path.display()).into());
        }
    } else {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn download_page(url: Url, output_path: &Path) -> Result<()> {
    if output_path.exists() {
        return Ok(());
    }

    println!("Downloading {}", url);
    let duration = Duration::from_secs_f64(random::<f64>() * 15.0 + 5.0);
    println!("Sleeping for {} seconds...", duration.as_secs());
    thread::sleep(duration);

    if let Some(parent) = output_path.parent() {
        mkdir(parent)?;
    }

    let browser = Browser::new(
        LaunchOptions::default_builder()
            .fetcher_options(FetcherOptions::default())
            .build()?,
    )?;
    let tab = browser.wait_for_initial_tab()?;
    tab.set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36", None, None)?;
    tab.navigate_to(url.as_str())?;
    tab.wait_until_navigated()?;
    let source_object = tab.wait_for_element("html")?.call_js_fn(
        "function() { return this.outerHTML; }",
        vec![],
        false,
    )?;

    let source = match source_object.value.ok_or("js function returned no value")? {
        serde_json::value::Value::String(source) => source,
        _ => unreachable!(),
    };

    fs::write(&output_path, source)?;

    Ok(())
}

fn page(path: &str) -> Result<Url> {
    Ok(Url::parse(BASE_URL)?.join(path)?)
}

fn products_index_page() -> Result<Url> {
    page("/gb/en/products/")
}

fn product_page(category: &Category, product: &ProductInfo) -> Result<Url> {
    page(&format!(
        "/gb/en/products/{}/{}/",
        category.slug, product.slug
    ))
}

fn load_html(path: &Path) -> Result<Html> {
    let doc = fs::read_to_string(path)?;
    Ok(Html::parse_document(doc.as_str()))
}

fn get_text(el: ElementRef) -> String {
    el.children()
        .map(|node| {
            if let Node::Text(text) = node.value() {
                text.text.to_string()
            } else {
                "".to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .to_owned()
        .replace("\u{a0}", " ")
}

pub fn download_products_index(output_dir: &Path) -> Result<PathBuf> {
    let output_path = output_dir.join("index.html");
    download_page(products_index_page()?, &output_path)?;
    Ok(output_path)
}

pub fn download_product_page(
    category: &Category,
    product: &ProductInfo,
    output_dir: &Path,
) -> Result<PathBuf> {
    let output_path = output_dir.join(&format!("{}/{}.html", category.slug, product.slug));
    download_page(product_page(category, product)?, &output_path)?;
    Ok(output_path)
}

pub fn parse_products_index(index_path: &Path) -> Result<Vec<Category>> {
    let html = load_html(index_path)?;

    let menu = select!(html, "li.with-mega-menu > div.products-mm > div.container")
        .next()
        .ok_or("failed to find main menu")?;

    let mut categories = vec![];

    for row in select!(menu, "ul.row") {
        for li in select!(row, "li") {
            let heading_link = select!(li, "h4 > a")
                .next()
                .ok_or("failed to find heading")?;

            let name = heading_link.inner_html().trim().to_owned();
            let mut category_href = page(
                heading_link
                    .value()
                    .attr("href")
                    .ok_or("failed to find href")?,
            )?;

            let slug = products_index_page()?
                .make_relative(&category_href)
                .ok_or("failed to make relative")?;

            // this allows us to get the slug for each product later.
            category_href.set_path(format!("{}/", category_href.path()).as_str());

            let mut category = Category {
                slug,
                name,
                products: vec![],
            };

            for product in select!(li, "div > a") {
                let name = product.inner_html().trim().to_owned();
                let href = page(product.value().attr("href").ok_or("failed to find href")?)?;
                let slug = category_href
                    .make_relative(&href)
                    .ok_or("failed to make relative")?;
                category.products.push(ProductInfo {
                    slug,
                    category_slug: category.slug.clone(),
                    name,
                });
            }

            categories.push(category);
        }
    }

    Ok(categories)
}

pub fn parse_product_page(page_path: &Path, info: &ProductInfo) -> Result<Product> {
    let html = load_html(page_path)?;

    let product_wrap = select!(html, "div.product-wrap")
        .next()
        .ok_or("failed to find product wrap")?;

    let product_wrap_container = select!(product_wrap, "div.product-wrap-container")
        .next()
        .ok_or("failed to find product wrap container")?;

    let showcase_img_src = Url::parse(
        select!(product_wrap_container, "div.showcase > img")
            .next()
            .ok_or("failed to find showcase img")?
            .value()
            .attr("src")
            .ok_or("failed to find showcase img src")?,
    )?;

    let detail = select!(product_wrap_container, "div.detail")
        .next()
        .ok_or("failed to find detail")?;

    let icon_img_src = Url::parse(
        select!(detail, "img")
            .next()
            .ok_or("failed to find detail img")?
            .value()
            .attr("src")
            .ok_or("failed to find icon img src")?,
    )?;

    let detail_text = select!(detail, "div.text")
        .next()
        .ok_or("failed to find detail text")?;

    let full_name = get_text(
        select!(detail_text, "h1 > strong")
            .next()
            .ok_or("failed to find full name")?,
    );

    let description = select!(detail_text, "p")
        .map(|p| get_text(p))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    let flavour_profile = get_text(
        select!(detail_text, "h2 > em")
            .next()
            .ok_or("failed to find flavour profile")?,
    )
    .replace("Flavor Profile: ", "");

    let tagline = get_text(
        select!(detail, "p.tagline")
            .next()
            .ok_or("failed to find tagline")?,
    );

    Ok(Product {
        slug: info.slug.clone(),
        category_slug: info.category_slug.clone(),
        name: info.name.clone(),
        full_name: full_name,
        description: description,
        tagline: tagline,
        flavour_profile: flavour_profile,
        icon_url: icon_img_src,
        showcase_image_url: showcase_img_src,
    })
}
