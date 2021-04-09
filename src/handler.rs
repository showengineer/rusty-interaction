use crate::security::*;
use crate::types::*;
use actix_web::http::{StatusCode};
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result,};
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use ed25519_dalek::{PublicKey};
use env_logger;

/// Macro that generates an `HttpResponse` containing a message serialized in JSON
macro_rules! ERROR_RESPONSE {
    ($status:expr, $message:expr) => {
        let emsg = MessageError::new(String::from($message));
        
        return Ok(HttpResponse::build(StatusCode::from_u16($status).unwrap())
            .content_type("application/json")
            .json(emsg));
    };
}

#[derive(Clone, Copy)]
pub struct InteractionHandler {
    app_public_key: PublicKey,
}

impl InteractionHandler {
    pub fn new(pbk_bytes: [u8; PUBLIC_KEY_LENGTH]) -> InteractionHandler {
        let pbk = PublicKey::from_bytes(&pbk_bytes);
        if pbk.is_err() {
            panic!("Failed to convert public key.");
        }
        return InteractionHandler {
            app_public_key: pbk.unwrap(),
        };
    }
    pub async fn interaction(self, req: HttpRequest, body: web::Bytes) -> Result<HttpResponse> {
        // Check for good content type --> must be application/json
        let ct = get_content_type(&req);
        if ct.is_some() {
            if ct.unwrap() != "application/json" {
                ERROR_RESPONSE!(400, "Bad Content-Type");
            }
        } else {
            ERROR_RESPONSE!(400, "Bad Content-Type");
        }
        let se = get_sig_ed25519(&req);
        let st = get_sig_timestamp(&req);

        // Check if proper headers are present. If none, reject.
        if se.is_none() || st.is_none() {
            ERROR_RESPONSE!(400, "Bad signature data");
        }

        // TODO: Domain check might be a good one.

        // Get request body
        let sta = String::from(std::str::from_utf8(&body).unwrap());

        // Verify timestamp + body against given signature
        match verify_discord_message(self.app_public_key, se.unwrap(), st.unwrap(), &sta) {
            // Verification failed, reject.
            // TODO: Switch error response
            Err(_) => {
                ERROR_RESPONSE!(401, "Invalid request signature");
            }

            // Signature OK. Continue with interaction processing.
            Ok(()) => {}
        }

        // TODO: Serialize and process interaction
        let response = InteractionResponse::new(InteractionResponseType::PONG, None);
        return Ok(HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .json(response));
    }
    pub async fn run(self) -> std::io::Result<()>{
        HttpServer::new(move || {
            App::new()
            .data(self)
            .route("/api/discord/interactions", web::post().to(|data: web::Data<InteractionHandler>, req: HttpRequest, body: web::Bytes| async move{
                data.interaction(req, body).await
            }))
        })
        .bind("127.0.0.1:80")?
        .run()
        .await
    }
}

/// Gets the Content-Type header from a `HttpRequest`
fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Content-Type")?.to_str().ok()
}

/// Gets the X-Signature-Ed25519 header from a `HttpRequest`
fn get_sig_ed25519<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("X-Signature-Ed25519")?.to_str().ok()
}

/// Gets the X-Signature-Timestamp header from a `HttpRequest`
fn get_sig_timestamp<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("X-Signature-Timestamp")?.to_str().ok()
}
