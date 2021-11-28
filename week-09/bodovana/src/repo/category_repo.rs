use std::sync::Arc;

use bodovana::{schema, PgPool, models::{Category, NewCategory}};

use anyhow::Result;
use diesel::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait CategoryRepo {
    async fn create_category(&self, name: &str, parent_id: Option<i32>) -> Result<i32>;
    async fn get_category(&self, category_id: i32) -> Result<Category>;
    async fn list_categories(&self) -> Result<Vec<Category>>;
}

pub struct PostgerCategoryRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgerCategoryRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self {
            pg_pool: pg_pool,
        }
    }
}

#[async_trait]
impl CategoryRepo for PostgerCategoryRepo {
    async fn create_category(&self, name: &str, parent_id: Option<i32>) -> Result<i32> {
        use schema::categories;
    
        let new_category = NewCategory {
            name: name,
            parent_id: parent_id
        };
    
        let rec: Category = diesel::insert_into(categories::table)
            .values(&new_category)
            .get_result(&self.pg_pool.get()?)
            .expect("Error saving new category");

        Ok(rec.id)
    }

    async fn get_category(&self, category_id: i32) -> Result<Category> {
        use self::schema::categories::dsl::*;

        Ok(
            categories
                .filter(id.eq(category_id))
                .first(&self.pg_pool.get()?)
                .expect("Error loading user")
        )
    }

    async fn list_categories(&self) -> Result<Vec<Category>> {
        use self::schema::categories::dsl::*;

        Ok(
            categories
                .load(&self.pg_pool.get()?)
                .expect("Error loading categories")
        )
    }
}
