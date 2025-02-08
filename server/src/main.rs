use std::fs::{self, File, metadata};
use std::io::Write;
use std::path::Path;
use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{web, App, HttpServer, HttpResponse, Error};
use futures::StreamExt;
use image::GenericImageView;
use serde_json::json;

async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let upload_dir = "./tmp";

    if !Path::new(upload_dir).exists() {
        fs::create_dir_all(upload_dir).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to create dir: {}", e))
        })?;
    }

    let mut response = vec![];

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Multipart error: {}", e))
        })?;

        let content_type = field.content_disposition().ok_or_else(|| {
            actix_web::error::ErrorBadRequest("Missing content disposition")
        })?;

        let filename = content_type.get_filename().ok_or_else(|| {
            actix_web::error::ErrorBadRequest("Missing filename")
        })?.to_string();
        
        let filepath = format!("{}/{}", upload_dir, filename);

        let mut file = File::create(&filepath).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("File create error: {}", e))
        })?;
        
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Chunk read error: {}", e))
            })?;
            file.write_all(&data).map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("File write error: {}", e))
            })?;
        }

        // 元の画像サイズとバイト数を取得
        let original_size = get_image_dimensions(&filepath).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Get original size error: {}", e))
        })?;
        let original_bytes = metadata(&filepath)?.len();

        // 最適化処理
        let optimized_size = optimize_file(&filepath).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Optimize error: {}", e))
        })?;
        let optimized_bytes = metadata(&filepath)?.len();

        let bytes_saved = original_bytes as i64 - optimized_bytes as i64;

        response.push(json!({
            "file": filename,
            "original_size": format!("{}x{}", original_size.0, original_size.1),
            "optimized_size": format!("{}x{}", optimized_size.0, optimized_size.1),
            "original_bytes": original_bytes,
            "optimized_bytes": optimized_bytes,
            "bytes_saved": bytes_saved
        }));
    }

    Ok(HttpResponse::Ok().json(response))
}

// 画像サイズを取得する関数
fn get_image_dimensions(filepath: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let img = image::open(filepath)?;
    Ok(img.dimensions())
}

fn optimize_file(filepath: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    // 画像最適化処理
    let img = image::open(filepath).map_err(|e| format!("Image open error: {}", e))?;
    let resized = img.resize(800, 600, image::imageops::FilterType::Lanczos3);
    resized.save(filepath).map_err(|e| format!("Image save error: {}", e))?;

    Ok(resized.dimensions()) // 新しいサイズを返す
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
        .wrap(
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header(),
        )
        .route("/api/upload", web::post().to(upload_file)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
