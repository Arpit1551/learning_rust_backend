use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use crate::middleware::auth::Claims;

const MAX_FILE_SIZE: usize = 2 * 1024 * 1024;
const ALLOWED_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp"];

pub async fn upload_file(
    req: HttpRequest,
    mut payload: Multipart
) -> HttpResponse {

    let claims = match req.extension().get::<Claims>().cloned() {
          Some(c) => c,
        None => return HttpResponse::Unauthorized().json(
            serde_json::json!({"error": "Unauthorized"})
        ),
    };

    
}