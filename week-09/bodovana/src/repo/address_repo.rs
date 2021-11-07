use std::sync::Arc;

use bodovana::{schema, PgPool, models::{Address, NewAddress}};

use anyhow::Result;
use diesel::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait AddressRepo {
    async fn create_address(&self, city: &str, street: &str, number: i32, zip: &str) -> Result<i32>;
    async fn get_address(&self, address_id: i32) -> Result<Address>;
    async fn list_addresses(&self) -> Result<Vec<Address>>;
}

pub struct PostgerAddressRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgerAddressRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self {
            pg_pool: pg_pool,
        }
    }
}

#[async_trait]
impl AddressRepo for PostgerAddressRepo {
    async fn create_address(&self, city: &str, street: &str, number: i32, zip: &str) -> anyhow::Result<i32> {
        use schema::address;

        let new_ticket = NewAddress {
            city: city,
            street: street,
            number: number,
            zip: zip,
        };

        let rec: Address = diesel::insert_into(address::table)
            .values(&new_ticket)
            .get_result(&self.pg_pool.get()?)
            .expect("Error saving new address");

        Ok(rec.id)
    }

    async fn get_address(&self, address_id: i32) -> Result<Address> {
        use self::schema::address::dsl::*;

        Ok(
            address
                .filter(id.eq(address_id))
                .first(&self.pg_pool.get()?)
                .expect("Error getting address")
        )
    }

    async fn list_addresses(&self) -> Result<Vec<Address>> {
        use self::schema::address::dsl::*;

        Ok(
            address
                // .filter(published.eq(true))
                // .limit(5)
                .load::<Address>(&self.pg_pool.get()?)
                .expect("Error loading addresses")
        )
    }
}
