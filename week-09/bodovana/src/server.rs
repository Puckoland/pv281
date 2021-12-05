use std::sync::Arc;
use grpc_handlers::MyCategoryService;
use grpc_handlers::MyProductService;
use tonic::transport::Server;
use bodovana::establish_connection;

mod templates;
mod grpc_handlers;
mod repo;

use repo::address_repo::*;
use repo::order_repo::*;
use repo::category_repo::*;
use repo::product_repo::*;
use repo::user_repo::*;

pub mod product {
    tonic::include_proto!("product");
}

pub mod category {
    tonic::include_proto!("category");
}

use category::category_service_server::CategoryServiceServer;
use product::product_service_server::ProductServiceServer;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let conn_pool = Arc::new(establish_connection().await);

    let _address_repo = Arc::new(PostgerAddressRepo::new(conn_pool.clone()));
    let _order_repo = Arc::new(PostgerOrderRepo::new(conn_pool.clone()));
    let category_repo = Arc::new(PostgerCategoryRepo::new(conn_pool.clone()));
    let product_repo = Arc::new(PostgresProductRepo::new(conn_pool.clone()));
    let _user_repo = Arc::new(PostgerUserRepo::new(conn_pool.clone()));

    let product_service = MyProductService::new(product_repo.clone());
    let category_service = MyCategoryService::new(category_repo.clone());

    Server::builder()
        .add_service(ProductServiceServer::new(product_service))
        .add_service(CategoryServiceServer::new(category_service))
        .serve("127.0.0.1:50051".parse()?)
        .await?;

    Ok(())
}