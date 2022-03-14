use monsterapi::api::{views, DBConn};
use rocket::{launch, routes};

#[launch]
pub fn launch() -> _ {
    rocket::build().attach(DBConn::fairing()).mount(
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
}
