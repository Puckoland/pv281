fn main() {
    compile_proto("proto/product.proto");
    compile_proto("proto/category.proto");
}

fn compile_proto(path: &str) {
    tonic_build::compile_protos(path).unwrap_or_else(|e| panic!("Failed to compile proto {:?}", e))
}
