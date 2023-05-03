use std::path::PathBuf;

use yt_buddy_core::DocumentLoader;
use yt_buddy_core::PdfFileLoader;

#[tokio::main]
async fn main() {
    println!("Example: pdf_loader_01");

    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let file_path = PathBuf::from(root_dir)
        .join("resources")
        .join("fek-5130-b-2022-agrodiatrofh.pdf");
    println!("Loading: {file_path:?}");

    let loader = PdfFileLoader::new(file_path);
    let res = loader.load().await.expect("Failed to load pdf");

    println!("res: {:?}", res);
}
