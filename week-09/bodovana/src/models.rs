use diesel::Queryable;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name="address"]
pub struct NewAddress<'a> {
    pub city: &'a str,
    pub street: &'a str,
    pub number: i32,
    pub zip: &'a str
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Address {
    pub id: i32,
    pub city: String,
    pub street: String,
    pub number: i32,
    pub zip: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub address_id: i32
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub address_id: i32
}

#[derive(Insertable)]
#[table_name="categorizations"]
pub struct NewCategorization {
    pub product_id: i32,
    pub category_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Categorization {
    pub id: i32,
    pub product_id: i32,
    pub category_id: i32,
}

#[derive(Insertable)]
#[table_name="products"]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub parent_id: Option<i32>
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>
}

#[derive(Insertable)]
#[table_name="orders"]
pub struct NewOrder {
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub street: String,
    pub number: i32,
    pub zip: String,
    pub name: String,
    pub price: i32,
    pub state_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Order {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub street: String,
    pub number: i32,
    pub zip: String,
    pub name: String,
    pub price: i32,
    pub state_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct MyState {
    pub id: i32,
    pub my_state: String,
}
