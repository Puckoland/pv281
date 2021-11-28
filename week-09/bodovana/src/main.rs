use std::sync::Arc;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use bodovana::establish_connection;

mod templates;
mod handlers;
mod repo;

use repo::address_repo::*;
use repo::order_repo::*;
use repo::category_repo::*;
use repo::product_repo::*;
use repo::user_repo::*;
use handlers::AppState;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let conn_pool = Arc::new(establish_connection().await);

    let address_repo = Arc::new(PostgerAddressRepo::new(conn_pool.clone()));
    let order_repo = Arc::new(PostgerOrderRepo::new(conn_pool.clone()));
    let category_repo = Arc::new(PostgerCategoryRepo::new(conn_pool.clone()));
    let product_repo = Arc::new(PostgresProductRepo::new(conn_pool.clone()));
    let user_repo = Arc::new(PostgerUserRepo::new(conn_pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { selected_category: 1 }))
            .app_data(web::Data::new(address_repo.clone()))
            .app_data(web::Data::new(order_repo.clone()))
            .app_data(web::Data::new(category_repo.clone()))
            .app_data(web::Data::new(product_repo.clone()))
            .app_data(web::Data::new(user_repo.clone()))
            .service(handlers::get_products)
            .service(web::resource("/").route(web::get().to(handlers::index)))
            .service(web::resource("/categories/{id}/products").route(web::get().to(handlers::get_products_by_category)))
            // an example of how to serve static files (css in this case, just for a demonstration)
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await?;

    Ok(())
}