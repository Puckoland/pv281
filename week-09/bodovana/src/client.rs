use product::product_service_client::ProductServiceClient;
use category::category_service_client::CategoryServiceClient;

pub mod category {
    tonic::include_proto!("category");
}

pub mod product {
    tonic::include_proto!("product");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = ProductServiceClient::connect("http://127.0.0.1:50051").await?;
    let mut category_client = CategoryServiceClient::connect("http://127.0.0.1:50051").await?;

    // Create a category
    let request = tonic::Request::new(category::CreateCategoryRequest {
        name: "proto".to_string(),
    });
    let response = category_client.create_category(request).await?;
    println!("Response from server: {:?}", response);

    // Create a product
    let request = tonic::Request::new(product::CreateProductRequest {
        name: "Proto phone".to_string(),
        price: 7000,
    });
    let response = client.create_product(request).await?;
    println!("Response from server: {:?}", response);

    // List items
    println!("Categories: {:?}", category_client.list_categories(()).await?);
    println!("Products: {:?}", client.list_products(()).await?);

    Ok(())
}
