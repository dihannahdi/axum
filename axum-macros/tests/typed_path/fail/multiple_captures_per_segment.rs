use axum_macros::TypedPath;
use serde::Deserialize;

#[derive(TypedPath, Deserialize)]
#[typed_path("/files/{name}.{ext}")]
struct MyPath {
    name: String,
    ext: String,
}

fn main() {}
