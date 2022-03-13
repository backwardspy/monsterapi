use super::{views, DBConn};

pub fn launch() {
    rocket::ignite()
        .attach(DBConn::fairing())
        .mount(
            "/",
            routes![
                views::index,
                views::category_list,
                views::category_detail,
                views::category_products_list,
                views::product_list,
                views::product_detail
            ],
        )
        .launch();
}
