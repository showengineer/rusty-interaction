pub mod handler;
pub mod security;
pub mod types;

#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests;


use crate::security::*;
use crate::types::*;

use actix_web::http::{StatusCode};
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result,};
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use ed25519_dalek::{PublicKey};

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

    /// Entry point function for handling `Interactions`
    pub async fn interaction(self, req: HttpRequest, body: web::Bytes) -> Result<HttpResponse> {
        // Check for good content type --> must be application/json
        let ct = get_header(&req, "Content-Type");
        if ct.is_some() {
            if ct.unwrap() != "application/json" {
                ERROR_RESPONSE!(400, "Bad Content-Type");
            }
        } else {
            ERROR_RESPONSE!(400, "Bad Content-Type");
        }


        let se = get_header(&req, "X-Signature-Ed25519");
        let st = get_header(&req, "X-Signature-Timestamp");

        // Check if proper headers are present. If not, reject.
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

        // Security checks passed, try deserializing request to Interaction.
        match serde_json::from_slice::<Interaction>(&body){
            Err(e) => {
                ERROR_RESPONSE!(400, format!("Bad body: {}", e));
            },
            Ok(_v) =>{
                //TODO: Reponds with OK PONG, parse it to interaction handler
                let response = InteractionResponse::new(InteractionResponseType::PONG, None);
                return Ok(HttpResponse::build(StatusCode::OK)
                    .content_type("application/json")
                    .json(response));
            }
        }
        
    }

    /// This is a predefined function that starts an `actix_web::HttpServer` and binds `self.interaction` to `/api/discord/interacitons`. 
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

/// Simpler header getter from a HTTP request
fn get_header<'a>(req: &'a HttpRequest, header: &str) -> Option<&'a str>{
    req.headers().get(header)?.to_str().ok()
}