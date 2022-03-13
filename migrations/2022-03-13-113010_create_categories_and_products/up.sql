CREATE TABLE category (
    slug TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE product (
    slug TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    full_name TEXT NOT NULL,
    description TEXT NOT NULL,
    tagline TEXT NOT NULL,
    flavour_profile TEXT NOT NULL,
    icon_url TEXT NOT NULL,
    showcase_image_url TEXT NOT NULL,
    category_slug TEXT NOT NULL,
    FOREIGN KEY(category_slug) REFERENCES category(slug)
);
