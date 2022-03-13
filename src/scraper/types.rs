use url::Url;

pub struct ProductInfo {
    pub slug: String,
    pub category_slug: String,
    pub name: String,
}

pub struct Category {
    pub slug: String,
    pub name: String,
    pub products: Vec<ProductInfo>,
}

pub struct Product {
    pub slug: String,
    pub category_slug: String,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub tagline: String,
    pub flavour_profile: String,
    pub icon_url: Url,
    pub showcase_image_url: Url,
}
