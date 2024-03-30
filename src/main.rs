mod file_readers;
mod pdf;

use crate::{
    file_readers::s3,
    pdf::{
        document_parser::{load_pdfium, parse_document},
        image_generator::get_images_from_pdf,
    },
};

use clap::Parser;
use dotenv::dotenv;
use pdfium_render::prelude::*;
use std::{collections::HashMap, env};

fn get_env_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => panic!("{} not found in .env", key),
    }
}

/**
 * Export a PDF file to images in a given image format (only PNG and WebP are supported).
 * I have tried Jpeg but it gives weird output.
 */
async fn export_pdf_to_images(
    image_format: image::ImageFormat,
    key: &String,
    include_thumbnail: Option<bool>,
) -> Result<(), PdfiumError> {
    // Read the file from S3.
    let file = s3::read_file(&get_env_var("AWS_BUCKET"), key).await;

    println!("Transforming file to byte vector");
    let bytes = file.collect().await.expect("Failed to read file");
    let byte_vec = bytes.to_vec();

    // parse the document and get the images.
    let pdfium = load_pdfium();
    let document = parse_document(&pdfium, byte_vec);

    get_images_from_pdf(document, image_format, include_thumbnail);

    Ok(())
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    key: String,
    format: String,
    include_thumbnail: Option<bool>,
}

#[tokio::main]
async fn main() {
    let mut image_formats = HashMap::new();

    image_formats.insert("png", image::ImageFormat::Png);
    image_formats.insert("webp", image::ImageFormat::WebP);

    let args = Cli::parse();

    println!("Exporting PDF to images with format: {:?}", &args.format);

    let image_format = match image_formats.get(&args.format.as_str()) {
        Some(image_format) => *image_format,
        None => image::ImageFormat::Png,
    };

    dotenv().ok();

    export_pdf_to_images(
        image_format,
        &args.key,
        args.include_thumbnail.or_else(|| Some(false)),
    )
    .await
    .unwrap();

    println!("Exported PDF to images successfully!");
}
