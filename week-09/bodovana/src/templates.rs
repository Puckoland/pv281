use bodovana::models::Product;
use bodovana::models::Category;
use askama::Template;

#[derive(Template)]
#[template(path = "../templates/index.html")]
pub struct Index {
    pub(crate) products: Vec<Product>,
    pub(crate) categories: Vec<Category>,
    pub(crate) selected_category: i32,
}

#[derive(Template)]
#[template(path = "../templates/my_products.html")]
pub struct Products {
    pub(crate) products: Vec<Product>,
}

