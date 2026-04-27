use actix_web::{
    Error, HttpResponse,
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use actix_web::HttpMessage;
use crate::utils::jwt::decode_token;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|x| x.to_str().ok());

    match auth_header {
        None => Ok(req.into_response(
            HttpResponse::Unauthorized().body("Unauthorized!")
        )),
        Some(header_value) => {
            let token = header_value.replace("Bearer ", "");
            match decode_token(&token) {
                Some(claims) => {
                    req.extensions_mut().insert(claims);
                    next.call(req).await.map(|res| res.map_into_boxed_body())
                }
                None => Ok(req.into_response(
                    HttpResponse::Unauthorized().body("Invalid token!")
                )),
            }
        }
    }
}