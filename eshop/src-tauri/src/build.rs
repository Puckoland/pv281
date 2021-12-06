fn main() -> anyhow::Result<()> {
  tonic_build::configure()
    .build_server(false)
    .type_attribute("product.Product", "#[derive(serde::Deserialize)]")
    .type_attribute("product.Product", "#[derive(serde::Serialize)] #[serde(rename_all = \"camelCase\")]")
    .type_attribute("product.ProductsReply", "#[derive(serde::Serialize, serde::Deserialize)]")
    .type_attribute("product.CreateProductRequest", "#[derive(serde::Serialize, serde::Deserialize)]")
    .type_attribute("product.CreateProductReply", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile(
      &["proto/product.proto"],
      &["proto/"],
    )?;

  tauri_build::build();

  Ok(())
}
