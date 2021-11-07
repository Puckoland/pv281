use std::sync::Arc;

mod address_repo;
mod product_repo;
mod user_repo;

use bodovana::{PgPool, models::{MyState, NewOrder, Order}, schema};
use user_repo::*;
use address_repo::*;
use product_repo::*;

use anyhow::Result;
use diesel::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepo {
    async fn create_order(&self, user_id: i32, product_id: i32, state_id: i32) -> Result<i32>;
    async fn get_info(&self, order_id: i32) -> Result<String>;
}

pub struct PostgerOrderRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgerOrderRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self {
            pg_pool: pg_pool,
        }
    }
}

#[async_trait]
impl OrderRepo for PostgerOrderRepo {
    async fn create_order(&self, user_id: i32, product_id: i32, state_id: i32) -> Result<i32> {
        use schema::orders;

        let user_repo = PostgerUserRepo::new(self.pg_pool.clone());
        let address_repo = PostgerAddressRepo::new(self.pg_pool.clone());
        let product_repo = PostgresProductRepo::new(self.pg_pool.clone());
        let user = user_repo.get_user(user_id).await?;
        let product = product_repo.get_product(product_id).await?;
        let address = address_repo.get_address(user.address_id).await?;
        
        let new_order = NewOrder {
            first_name: user.first_name,
            last_name: user.last_name,
            city: address.city,
            street: address.street,
            number: address.number,
            zip: address.zip,
            name: product.name,
            price: product.price,
            state_id: state_id,
        };

        let rec: Order = diesel::insert_into(orders::table)
            .values(&new_order)
            .get_result(&self.pg_pool.get()?)
            .expect("Error saving new product");

        Ok(rec.id)
    }

    async fn get_info(&self, order_id: i32) -> Result<String> {
        use self::schema::orders;
        use self::schema::mystate;

        let (order, mystate): (Order, MyState) = orders::table
                .filter(orders::id.eq(order_id))
                .inner_join(mystate::table)
                .first(&self.pg_pool.get()?)
                .expect("Error generating invoice");
        let invoice = format!(
            "Name: {} {}\n\
            Address: {} {}, {} {}\n\
            Product: {}\n\
            Price: {}\n\
            State: {}",
            order.first_name, order.last_name, order.street, order.number, order.zip, order.city, order.name, order.price, mystate.my_state);

        Ok(invoice)
    }
}