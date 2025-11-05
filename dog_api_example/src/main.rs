use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiError {
    Network(String),
    Http(String),
    Parse(String),
    Download(String),
    FileIO(String),
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn fetch_random_dog_image() -> Result<DogImage, ApiError> {
    let url = "https://dog.ceo/api/breeds/image/random";

    let response = ureq::get(url)
        .call()
        .map_err(|e| ApiError::Network(format!("Request failed: {}", e)))?;

    if response.status() != 200 {
        return Err(ApiError::Http(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    response
        .into_json::<DogImage>()
        .map_err(|e| ApiError::Parse(format!("JSON parse failed: {}", e)))
}

fn download_image(url: &str, filename: &str) -> Result<(), ApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| ApiError::Network(format!("Image request failed: {}", e)))?;

    if response.status() != 200 {
        return Err(ApiError::Http(format!(
            "Image HTTP error: {}",
            response.status()
        )));
    }

    let mut reader = response.into_reader();
    let mut buffer = Vec::new();
    std::io::copy(&mut reader, &mut buffer)
        .map_err(|e| ApiError::Download(format!("Failed to read image data: {}", e)))?;

    let mut file =
        File::create(Path::new(filename)).map_err(|e| ApiError::FileIO(format!("{}", e)))?;
    file.write_all(&buffer)
        .map_err(|e| ApiError::FileIO(format!("Failed to write image: {}", e)))?;

    Ok(())
}

fn main() {
    println!("Dog Image Downloader 🐶");
    println!("========================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);

        match fetch_random_dog_image() {
            Ok(dog_image) => {
                let filename = format!("dog_image_{}.jpg", i);
                println!("✅ Got image URL: {}", dog_image.message);

                match download_image(&dog_image.message, &filename) {
                    Ok(_) => println!("📸 Saved as: {}", filename),
                    Err(e) => println!("❌ Download failed: {:?}", e),
                }
            }
            Err(e) => println!("❌ API Error: {:?}", e),
        }

        println!();
    }

    println!("All done!");
}
