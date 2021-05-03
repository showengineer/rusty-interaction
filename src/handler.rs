use crate::security::*;
use crate::types::interaction::*;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use reqwest::Client;

use log::{error, info};

use ed25519_dalek::PublicKey;

use rustls::ServerConfig;

use std::{collections::HashMap, future::Future, pin::Pin};

pub type HandlerResponse = InteractionResponse;

type HandlerFunction = fn(Context) -> Pin<Box<dyn Future<Output = HandlerResponse> + Send>>;

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
    handles: HashMap<&'static str, HandlerFunction>,
}

impl InteractionHandler {
    /// Initalizes a new `InteractionHandler`
    pub fn new(pbk_str: &str) -> InteractionHandler {
        let pbk_bytes =
            hex::decode(pbk_str).expect("Failed to parse the public key from hexadecimal");

        let app_public_key =
            PublicKey::from_bytes(&pbk_bytes).expect("Failed to parse public key.");

        InteractionHandler {
            app_public_key,
            client: Client::new(),
            handles: HashMap::new(),
        }
    }

    /// Binds an async function to a command.
    /// Your function must take a [`Context`] as an argument and must return a [`InteractionResponse`].
    /// Make sure to use the `#[slash_command]` procedural macro to make it usable for the handler.
    ///
    /// Like:
    /// ```rust
    /// # use rusty_interaction::types::interaction::{Context, InteractionResponse};
    /// # use attributes::slash_command;
    /// #[slash_command]
    /// async fn do_work(ctx: Context) -> InteractionResponse {
    ///     todo!("Do work and return a response")
    /// }
    /// ```
    /// # Example
    /// ```ignore
    /// # use rusty_interaction::types::interaction::{Context, InteractionResponse};
    /// # use rusty_interaction::handler::InteractionHandler;
    /// # use attributes::slash_command;
    /// const PUB_KEY: &str = "my_public_key";
    ///
    /// #[slash_command]
    /// async fn pong_handler(ctx: Context) -> InteractionResponse {
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
    pub fn add_command(&mut self, name: &'static str, func: HandlerFunction) {
        self.handles.insert(name, func);
    }

    /// Entry point function for handling `Interactions`
    pub async fn interaction(&self, req: HttpRequest, body: String) -> Result<HttpResponse> {
        // Check for good content type --> must be application/json

        if let Some(ct) = req.headers().get("Content-Type") {
            if ct != "application/json" {
                error!("BAD CONTENT");
                return ERROR_RESPONSE!(400, "Bad Content-Type");
            }
        } else {
            error!("BAD CONTENT");
            return ERROR_RESPONSE!(400, "Bad Content-Type");
        }

        let se = get_header(&req, "X-Signature-Ed25519");
        let st = get_header(&req, "X-Signature-Timestamp");

        // TODO: Domain check might be a good one.

        if let Some((se, st)) = se.zip(st) {
            // Verify timestamp + body against given signature
            if verify_discord_message(self.app_public_key, se, st, &body).is_ok() {
                // Signature OK. Continue with interaction processing.
            } else {
                // Verification failed, reject.
                // TODO: Switch error response

                error!("BAD SIGNATURE");
                return ERROR_RESPONSE!(401, "Invalid request signature");
            }
        } else {
            // If proper headers are not present reject.

            error!("MISSING HEADERS");
            return ERROR_RESPONSE!(400, "Bad signature data");
        }

        // Security checks passed, try deserializing request to Interaction.
        match serde_json::from_str::<Interaction>(&body) {
            Err(e) => {
                error!("BAD FORM: {:?}. Error: {}", body, e);
                return ERROR_RESPONSE!(400, format!("Bad body: {}", e));
            }
            Ok(interaction) => {
                if interaction.r#type == InteractionType::Ping {
                    let response = InteractionResponse::new(InteractionResponseType::Pong, None);
                    info!("RESP: PONG");
                    return Ok(HttpResponse::build(StatusCode::OK)
                        .content_type("application/json")
                        .json(response));
                }

                let data = if let Some(ref data) = interaction.data {
                    data
                } else {
                    return ERROR_RESPONSE!(500, "Failed to unwrap");
                };

                if let Some(handler) = self.handles.get(data.name.as_str()) {
                    // do stuff with v if needed

                    // construct a Context
                    let ctx = Context::new(self.client.clone(), interaction);

                    // Call the handler
                    let response = handler(ctx).await;

                    if response.r#type == InteractionResponseType::DefferedChannelMessageWithSource{
                        // The use of HTTP Code 202 is more appropiate when a Interaction is deffered. 
                        Ok(HttpResponse::build(StatusCode::ACCEPTED).json(response))
                    }
                    else{
                        // Send out a response to Discord
                        Ok(HttpResponse::build(StatusCode::OK).json(response))
                    }
                } else {
                    ERROR_RESPONSE!(500, "No associated handler found")
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
                    |data: web::Data<InteractionHandler>, req: HttpRequest, body: String| async move {
                        data.interaction(req, body).await
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
                    |data: web::Data<InteractionHandler>, req: HttpRequest, body: String| async move {
                        data.interaction(req, body).await
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
