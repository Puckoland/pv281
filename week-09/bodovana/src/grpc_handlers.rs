use crate::repo::category_repo::{CategoryRepo, PostgerCategoryRepo};
use crate::repo::product_repo::{ProductRepo, PostgresProductRepo};
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::category;
use crate::category::category_service_server::CategoryService;
use crate::category::{CategoriesReply, CreateCategoryReply, CreateCategoryRequest};
use crate::product;
use crate::product::product_service_server::ProductService;
use crate::product::{CreateProductReply, CreateProductRequest, ProductsReply};

pub struct MyProductService {
    data: Arc<PostgresProductRepo>,
}

pub struct MyCategoryService {
    data: Arc<PostgerCategoryRepo>,
}

impl MyProductService {
    pub fn new(data: Arc<PostgresProductRepo>) -> MyProductService {
        MyProductService { data }
    }
}

impl MyCategoryService {
    pub fn new(data: Arc<PostgerCategoryRepo>) -> MyCategoryService {
        MyCategoryService { data }
    }
}

#[tonic::async_trait]
impl ProductService for MyProductService {
    async fn list_products(&self, request: Request<()>) -> Result<Response<ProductsReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let product_list = self.data.list_products().await.unwrap_or_default();

        let reply = ProductsReply {
            products: product_list
                .into_iter()
                .map(|product| product::Product {
                    id: product.id,
                    name: product.name,
                    price: product.price,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }

    async fn create_product(
        &self,
        request: Request<CreateProductRequest>,
    ) -> Result<Response<CreateProductReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let body = &request.into_inner();
        let result = self
            .data
            .create_product(&body.name, body.price, body.categories.clone())
            .await;

        let reply = match result {
            Ok(id) => CreateProductReply {
                message: format!("Product created with id {}!", id),
            },
            _ => return Err(Status::internal("Failed to create product!")),
        };

        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl CategoryService for MyCategoryService {
    async fn list_categories(
        &self,
        request: Request<()>,
    ) -> Result<Response<CategoriesReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let category_list = self.data.list_categories().await.unwrap_or_default();

        let reply = CategoriesReply {
            categories: category_list
                .into_iter()
                .map(|category| category::Category {
                    id: category.id,
                    name: category.name,
                    parent_id: category.parent_id,
                    // start_time: Some(category.start_time.convert()),
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }

    async fn create_category(
        &self,
        request: Request<CreateCategoryRequest>,
    ) -> Result<Response<CreateCategoryReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let body = &request.into_inner();
        let result = self.data.create_category(&body.name, body.parent_id).await;

        let reply = match result {
            Ok(id) => CreateCategoryReply {
                message: format!("Category created with id {}!", id),
            },
            _ => return Err(Status::internal("Failed to create category!")),
        };

        Ok(Response::new(reply))
    }
}
