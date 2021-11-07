#[path ="../repo/address_repo.rs"] pub mod address;
#[path ="../repo/order_repo.rs"] pub mod order;
#[path ="../repo/category_repo.rs"] pub mod category;
#[path ="../repo/product_repo.rs"] pub mod product;
#[path ="../repo/user_repo.rs"] pub mod user;

use std::sync::Arc;
use address::*;
use order::*;
use category::*;
use product::*;
use user::*;

use anyhow::Result;
use bodovana::PgPool;
use bodovana::establish_connection;
use futures::future::try_join_all;

async fn address(conn_pool: Arc<PgPool>) {
    let address_repo = PostgerAddressRepo::new(conn_pool);
    let addresses = [
        address_repo.create_address("Brno", "Botanicka", 5, "08213"),
        address_repo.create_address("Sabinov", "17. novembra", 1, "08301"),
        address_repo.create_address("Praha", "NeviemAka", 787, "12345")
    ];
    let addresses = try_join_all(addresses).await.unwrap();
    println!("{:?}", addresses);
}

async fn users(conn_pool: Arc<PgPool>) {
    let user_repo = PostgerUserRepo::new(conn_pool);
    let users = [
        user_repo.create_user("Filip", "Karnis", 2),
        user_repo.create_user("Janko", "Hrasko", 1),
        user_repo.create_user("Matko", "Klingac", 3)
    ];
    let users = try_join_all(users).await.unwrap();
    println!("{:?}", users);
}

async fn categories(conn_pool: Arc<PgPool>) {
    let category_repo = PostgerCategoryRepo::new(conn_pool);
    let categories = [
        category_repo.create_category("Phone", None),
        category_repo.create_category("Apple phone", Some(1)),
        category_repo.create_category("Other", None)
    ];
    let categories = try_join_all(categories).await.unwrap();
    println!("{:?}", categories);
}

async fn products(conn_pool: Arc<PgPool>) {
    let product_repo = PostgresProductRepo::new(conn_pool);
    let products = [
        product_repo.create_product("iPhone", 1000, vec![1, 3]),
        product_repo.create_product("iPad", 1300, vec![3]),
        product_repo.create_product("Samsung", 900, Vec::new())
    ];
    let products = try_join_all(products).await.unwrap();
    println!("{:?}", products);
}

async fn orders(conn_pool: Arc<PgPool>) {
    let order_repo = PostgerOrderRepo::new(conn_pool);
    let orders = [
        order_repo.create_order(1, 1, 1),
        order_repo.create_order(2, 1, 2),
        order_repo.create_order(3, 3, 3)
    ];
    let orders = try_join_all(orders).await.unwrap();
    println!("{:?}", orders);
}

async fn test_invoice(conn_pool: Arc<PgPool>) -> Result<()> {
    let order_repo = PostgerOrderRepo::new(conn_pool.clone());
    println!("{}", order_repo.get_info(1).await?);
    let user_repo = PostgerUserRepo::new(conn_pool.clone());
    let address_repo = PostgerAddressRepo::new(conn_pool.clone());
    let product_repo = PostgresProductRepo::new(conn_pool.clone());
    let address_id = address_repo.create_address("Presov", "Ruzova", 10, "08000").await?;
    user_repo.update_address(1, address_id).await?;
    product_repo.update_price(1, 1200).await?;
    println!("{}", order_repo.get_info(1).await?);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn_pool = Arc::new(establish_connection().await);
    address(conn_pool.clone()).await;
    users(conn_pool.clone()).await;
    categories(conn_pool.clone()).await;
    products(conn_pool.clone()).await;
    orders(conn_pool.clone()).await;
    test_invoice(conn_pool.clone()).await?;

    Ok(())
}