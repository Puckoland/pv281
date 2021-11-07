use std::sync::Arc;

use bodovana::{schema, PgPool, models::{NewUser, User}};

use anyhow::Result;
use diesel::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo {
    async fn create_user(&self, first_name: &str, last_name: &str, address_id: i32) -> Result<i32>;
    async fn get_user(&self, user_id: i32) -> Result<User>;
    async fn update_address(&self, user_id: i32, address_id: i32) -> Result<User>;
}

pub struct PostgerUserRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgerUserRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self {
            pg_pool: pg_pool,
        }
    }
}

#[async_trait]
impl UserRepo for PostgerUserRepo {
    async fn create_user(&self, first_name: &str, last_name: &str, address_id: i32) -> anyhow::Result<i32> {
        use schema::users;

        let new_user = NewUser {
            first_name: first_name,
            last_name: last_name,
            address_id: address_id
        };

        let rec: User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&self.pg_pool.get()?)
            .expect("Error saving new user");

        Ok(rec.id)
    }

    async fn get_user(&self, user_id: i32) -> Result<User> {
        use self::schema::users::dsl::*;

        Ok(
            users
                .filter(id.eq(user_id))
                .first(&self.pg_pool.get()?)
                .expect("Error loading user")
        )
    }

    async fn update_address(&self, user_id: i32, new_address_id: i32) -> Result<User> {
        use self::schema::users::dsl::*;
        
        let user: User = diesel::update(users.filter(id.eq(user_id)))
            .set(address_id.eq(new_address_id))
            .get_result(&self.pg_pool.get()?)
            .expect("Error updating user");
        
        Ok(user)
    }
}
