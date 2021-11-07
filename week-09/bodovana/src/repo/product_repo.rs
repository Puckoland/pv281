use std::sync::Arc;

use bodovana::{schema, PgPool, models::{Categorization, NewCategorization, NewProduct, Product}};

use anyhow::Result;
use diesel::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepo {
    async fn create_product(&self, name: &str, price: i32, category_ids: Vec<i32>) -> Result<i32>;
    async fn get_product(&self, product_id: i32) -> Result<Product>;
    async fn update_price(&self, product_id:i32, price: i32) -> Result<Product>;
}

pub struct PostgresProductRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresProductRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl ProductRepo for PostgresProductRepo {
    async fn create_product(&self, name: &str, price: i32, category_ids: Vec<i32>) -> Result<i32> {
        use schema::products;
        use schema::categorizations;
    
        let new_product = NewProduct {
            name: name,
            price: price,
        };
    
        let rec: Product = diesel::insert_into(products::table)
            .values(&new_product)
            .get_result(&self.pg_pool.get()?)
            .expect("Error saving new product");
        category_ids.iter().for_each(|cat| {
            let new_categorization = NewCategorization {
                category_id: cat.clone(),
                product_id: rec.id,
            };
            let _: Categorization = diesel::insert_into(categorizations::table)
                .values(&new_categorization)
                .get_result(&self.pg_pool.get().unwrap())
                .expect("Error saving new categorization");
            }
        );

        Ok(rec.id)
    }

    async fn get_product(&self, product_id: i32) -> Result<Product> {
        use self::schema::products::dsl::*;

        Ok(
            products
                .filter(id.eq(product_id))
                .first(&self.pg_pool.get()?)
                .expect("Error loading product")
        )
    }

    async fn update_price(&self, product_id: i32, new_price: i32) -> Result<Product> {
        use self::schema::products::dsl::*;

        let product: Product = diesel::update(products.filter(id.eq(product_id)))
            .set(price.eq(new_price))
            .get_result(&self.pg_pool.get()?)
            .expect("Error updating product");

        Ok(product)
    }
}