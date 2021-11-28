use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;
use std::sync::Arc;

use crate::templates::{Index, Products};
use crate::repo::category_repo::*;
use crate::repo::product_repo::*;

#[get("/products")]
pub async fn get_products(data: web::Data<Arc<PostgresProductRepo>>) -> impl Responder {
    let products = data.list_products().await.unwrap_or_default();

    HttpResponse::Ok().json(products)
}

#[get("/categories")]
pub async fn get_categories(data: web::Data<Arc<PostgerCategoryRepo>>) -> impl Responder {
    let categories = data.list_categories().await.unwrap_or_default();

    HttpResponse::Ok().json(categories)
}

pub async fn get_products_by_category(path: web::Path<i32>, data: web::Data<Arc<PostgresProductRepo>>) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let products = data.list_products_by_category(id).await.unwrap_or_default();

    let template = Products { products };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

// This struct represents state
pub struct AppState {
    pub selected_category: i32,
}

// #[post("/product")]
// pub async fn add_product(
//     data: web::Data<Arc<PostgresProductRepo>>,
//     body: web::Json<ProductPostData>,
// ) -> impl Responder {
//     let result = data
//         .add_product(body.first_name.clone(), body.last_name.clone())
//         .await;

//     match result {
//         Ok(_) => HttpResponse::Created(),
//         Err(_) => HttpResponse::InternalServerError(),
//     }
// }

pub async fn index(
    data: web::Data<AppState>,
    product_data: web::Data<Arc<PostgresProductRepo>>,
    category_data: web::Data<Arc<PostgerCategoryRepo>>,
) -> actix_web::Result<HttpResponse> {
    let selected_category = data.selected_category;
    let products = product_data.list_products_by_category(selected_category).await.unwrap_or_default();
    let categories = category_data.list_categories().await.unwrap_or_default();
    let template = Index { products, categories, selected_category };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}
