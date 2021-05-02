use crate::security::*;
use crate::types::{interaction::*, MessageError};

use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use reqwest::Client;

use ed25519_dalek::PUBLIC_KEY_LENGTH;

use log::{error, info};

macro_rules! ERROR_RESPONSE {
    ($status:expr, $message:expr) => {
        let emsg = MessageError::new(String::from($message));

        return Ok(HttpResponse::build(StatusCode::from_u16($status).unwrap())
            .content_type("application/json")
            .json(emsg));
    };
}

use ed25519_dalek::PublicKey;
use hex;

use rustls::ServerConfig;

use std::boxed::Box;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub type HandlerResponse = InteractionResponse;

#[derive(Clone)]
/// The InteractionHandler is the 'thing' that will handle your incoming interactions.
/// It does interaction validation (as required by Discord) and provides a pre-defined actix-web server
/// with [`InteractionHandler::run`] and [`InteractionHandler::run_ssl`]
pub struct InteractionHandler {
    /// The public key of your application.
    pub app_public_key: PublicKey,
    client: Client,
    /// Your bot token or bearer
    //auth_key: &'static str,
    // Might want to change this to use the command id rather than the name of the command: prone to duplicates.
    handles:
        HashMap<&'static str, fn(Context) -> Pin<Box<dyn Future<Output = HandlerResponse> + Send>>>,
}

impl InteractionHandler {
    #[allow(unused_assignments)]
    /// Initalizes a new `InteractionHandler`
    pub fn new(pbk_str: &str) -> InteractionHandler {
        let bytes = hex::decode(pbk_str);

        // Init a normal array.
        let mut pbk_bytes: [u8; PUBLIC_KEY_LENGTH] = [0; PUBLIC_KEY_LENGTH];

        match bytes {
            Err(_) => panic!("Failed to parse the public key"),
            Ok(k) => {
                if k.len() != PUBLIC_KEY_LENGTH {
                    panic!("Failed to parse the public key (bad length)");
                }
                pbk_bytes = convert_to_arr::<u8, PUBLIC_KEY_LENGTH>(k);
            }
        };
        let pbk = PublicKey::from_bytes(&pbk_bytes);
        if pbk.is_err() {
            panic!("Failed to convert public key.");
        }
        return InteractionHandler {
            app_public_key: pbk.unwrap(),
            client: Client::new(),
            handles: HashMap::new(),
        };
    }

    /// Binds an async function to a command.
    /// Your function must take a [`Context`] as an argument and must return a [`InteractionResponse`].
    /// Make sure to use the `#[slash_command]` procedural macro to make it usable for the handler.
    ///
    /// Like:
    /// ```rust
    /// #[slash_command]
    /// async fn do_work(ctx: Context) -> InteractionResponse{
    ///     // Do work and return a response
    /// }
    /// ```
    /// # Example
    /// ```rust
    /// const PUB_KEY: &str = "my_public_key"
    ///
    /// #[slash_command]
    /// async fn pong_handler(ctx: Context) -> InteractionResponse{
    ///     return ctx.respond()
    ///             .content("Pong!")
    ///             .finish();
    /// }
    ///
    /// #[actix_web::main]
    /// async fn main() -> std::io::Result<()> {
    ///
    ///     let mut handle = InteractionHandler::new(PUB_KEY);

    ///     handle.add_command("ping", pong_handler);
    ///     

    ///     return handle.run().await;
    /// }
    /// ```
    pub fn add_command(
        &mut self,
        name: &'static str,
        func: fn(Context) -> Pin<Box<dyn Future<Output = HandlerResponse> + Send>>,
    ) {
        self.handles.insert(name, func);
    }

    /// Entry point function for handling `Interactions`
    pub async fn interaction(&self, req: HttpRequest, body: web::Bytes) -> Result<HttpResponse> {
        // Check for good content type --> must be application/json
        let ct = get_header(&req, "Content-Type");
        if ct.is_some() {
            if ct.unwrap() != "application/json" {
                error!("BAD CONTENT");
                ERROR_RESPONSE!(400, "Bad Content-Type");
            }
        } else {
            error!("BAD CONTENT");
            ERROR_RESPONSE!(400, "Bad Content-Type");
        }

        let se = get_header(&req, "X-Signature-Ed25519");
        let st = get_header(&req, "X-Signature-Timestamp");

        // Check if proper headers are present. If not, reject.
        if se.is_none() || st.is_none() {
            error!("NO HEADERS");
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
                error!("BAD SIGNATURE");
                ERROR_RESPONSE!(401, "Invalid request signature");
            }

            // Signature OK. Continue with interaction processing.
            Ok(()) => {}
        }

        // Security checks passed, try deserializing request to Interaction.
        match serde_json::from_slice::<Interaction>(&body) {
            Err(e) => {
                error!("BAD FORM: {:?}. Error: {}", body, e);
                ERROR_RESPONSE!(400, format!("Bad body: {}", e));
            }
            Ok(v) => {
                if v.r#type == InteractionType::PING {
                    let response = InteractionResponse::new(InteractionResponseType::PONG, None);
                    info!("RESP: PONG");
                    return Ok(HttpResponse::build(StatusCode::OK)
                        .content_type("application/json")
                        .json(response));
                } else {
                    if v.data.is_none() {
                        ERROR_RESPONSE!(500, "Failed to unwrap");
                    }

                    let dat = v.clone().data.unwrap();

                    match self.handles.get(dat.name.as_str()) {
                        Some(f) => {
                            let cpy = v.clone();

                            // construct a Context
                            let ctx = Context::new(self.client.clone(), cpy);
                            // Call the handler
                            let r = f(ctx).await;

                            // do stuff with v if needed

                            // Send out a response to Discord
                            return Ok(HttpResponse::build(StatusCode::OK)
                                .content_type("application/json")
                                .json(r));
                        }
                        None => {
                            ERROR_RESPONSE!(500, "No associated handler found");
                        }
                    }
                }
            }
        }
    }

    /// This is a predefined function that starts an `actix_web::HttpServer` and binds `self.interaction` to `/api/discord/interacitons`.
    /// Note that you'll eventually have to switch to an HTTPS server. This function does not provide this.
    ///
    /// **The server runs on port 10080!**
    pub async fn run(self) -> std::io::Result<()> {
        HttpServer::new(move || {
            App::new().data(self.clone()).route(
                "/api/discord/interactions",
                web::post().to(
                    |data: web::Data<InteractionHandler>, req: HttpRequest, body: web::Bytes| {
                        let data = data.into_inner();
                        async move { (*data).clone().interaction(req, body).await }
                    },
                ),
            )
        })
        .bind("0.0.0.0:10080")?
        .run()
        .await
    }

    /// Same as [`InteractionHandler::run`] but starts a server with SSL/TLS.
    ///
    /// **The server runs on port 10443!**
    pub async fn run_ssl(self, server_conf: ServerConfig) -> std::io::Result<()> {
        HttpServer::new(move || {
            App::new().data(self.clone()).route(
                "/api/discord/interactions",
                web::post().to(
                    |data: web::Data<InteractionHandler>, req: HttpRequest, body: web::Bytes| {
                        let data = data.into_inner();
                        async move { (*data).clone().interaction(req, body).await }
                    },
                ),
            )
        })
        .bind_rustls("0.0.0.0:10443", server_conf)?
        .run()
        .await
    }
}

/// Simpler header getter from a HTTP request
fn get_header<'a>(req: &'a HttpRequest, header: &str) -> Option<&'a str> {
    req.headers().get(header)?.to_str().ok()
}
