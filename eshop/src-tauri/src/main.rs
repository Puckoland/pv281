#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod product {
  tonic::include_proto!("product");
}

use futures::lock::Mutex;
use crate::product::Product;
use product::product_service_client::ProductServiceClient;
use tauri::State;


pub struct AppState {
  client: Mutex<ProductServiceClient<tonic::transport::Channel>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let client = ProductServiceClient::connect("http://127.0.0.1:50051").await?;

  tauri::Builder::default()
    .manage(AppState {
      client: Mutex::new(client),
    })
    .invoke_handler(tauri::generate_handler![get_products, add_product])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
async fn get_products(state: State<'_, AppState>) -> Result<Vec<Product>, String> {
  let response = state
    .client
    .lock()
    .await
    .list_products(tonic::Request::new(()))
    .await;

  match response {
    Ok(response) => {
      return Ok(response.into_inner().products);
    }
    Err(_) => Err("Could not get products".to_string()),
  }
}

#[tauri::command]
async fn add_product(state: State<'_, AppState>, name: String, price: i32) -> Result<String, String> {
  let response = state
    .client
    .lock()
    .await
    .create_product(tonic::Request::new(product::CreateProductRequest {
      name,
      price,
      categories: vec![],
    }))
    .await;

  match response {
    Ok(response) => {
      return Ok(response.into_inner().message);
    }
    Err(_) => Err("Could not create product".to_string()),
  }
}
