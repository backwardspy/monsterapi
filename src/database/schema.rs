table! {
    category (slug) {
        slug -> Text,
        name -> Text,
    }
}

table! {
    product (slug) {
        slug -> Text,
        name -> Text,
        full_name -> Text,
        description -> Text,
        tagline -> Text,
        flavour_profile -> Text,
        icon_url -> Text,
        showcase_image_url -> Text,
        category_slug -> Text,
    }
}

joinable!(product -> category (category_slug));

allow_tables_to_appear_in_same_query!(category, product,);
