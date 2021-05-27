use crate::security::*;
use crate::types::interaction::*;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
#[cfg(feature = "extended-handler")]
use reqwest::header;
use reqwest::Client;

use log::{debug, error};

use ed25519_dalek::PublicKey;

use rustls::ServerConfig;

use std::{collections::HashMap, future::Future, pin::Pin};

/// Alias for InteractionResponse
pub type HandlerResponse = InteractionResponse;

type HandlerFunction = fn(Context) -> Pin<Box<dyn Future<Output = HandlerResponse> + Send>>;

#[derive(Clone, Debug)]
#[cfg(all(feature = "handler", not(feature = "extended-handler")))]
/// The InteractionHandler is the 'thing' that will handle your incoming interactions.
/// It does interaction validation (as required by Discord) and provides a pre-defined actix-web server
/// with [`InteractionHandler::run`] and [`InteractionHandler::run_ssl`]
pub struct InteractionHandler {
    /// The public key of your application.
    pub app_public_key: PublicKey,
    client: Client,
    // Might want to change this to use the command id rather than the name of the command: prone to duplicates.
    global_handles: HashMap<&'static str, HandlerFunction>,

    component_handles: HashMap<&'static str, HandlerFunction>,
}

#[derive(Clone, Debug)]
/// The InteractionHandler is the 'thing' that will handle your incoming interactions.
/// It does interaction validation (as required by Discord) and provides a pre-defined actix-web server
/// with [`InteractionHandler::run`] and [`InteractionHandler::run_ssl`]
#[cfg(feature = "extended-handler")]
pub struct InteractionHandler {
    /// The public key of your application.
    pub app_public_key: PublicKey,
    client: Client,
    
    global_handles: HashMap<&'static str, HandlerFunction>,
    component_handles: HashMap<&'static str, HandlerFunction>,
}

impl InteractionHandler {
    #[cfg(all(feature = "handler", not(feature = "extended-handler")))]
    /// Initalizes a new `InteractionHandler`
    pub fn new(pbk_str: &str) -> InteractionHandler {
        let pbk_bytes =
            hex::decode(pbk_str).expect("Failed to parse the public key from hexadecimal");

        let app_public_key =
            PublicKey::from_bytes(&pbk_bytes).expect("Failed to parse public key.");

        InteractionHandler {
            app_public_key,
            client: Client::new(),
            global_handles: HashMap::new(),
            component_handles: HashMap::new(),
        }
    }
    #[cfg(feature = "extended-handler")]
    /// Initalizes a new `InteractionHandler`
    pub fn new(pbk_str: &str, token: &'static str) -> InteractionHandler {
        let pbk_bytes =
            hex::decode(pbk_str).expect("Failed to parse the public key from hexadecimal");

        let app_public_key =
            PublicKey::from_bytes(&pbk_bytes).expect("Failed to parse public key.");

        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_static(token);
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        let new_c = Client::builder().default_headers(headers).build().unwrap();

        InteractionHandler {
            app_public_key,
            client: new_c,
            global_handles: HashMap::new(),
            component_handles: HashMap::new(),
        }
    }

    /// Binds an async function to a command.
    /// Your function must take a [`Context`] as an argument and must return a [`InteractionResponse`].
    /// Make sure to use the `#[slash_command]` procedural macro to make it usable for the handler.
    ///
    /// Like:
    /// ```ignore
    /// # use rusty_interaction::types::interaction::{Context, InteractionResponse};
    /// # use attributes::slash_command;
    /// #[slash_command]
    /// async fn do_work(ctx: Context) -> InteractionResponse {
    ///     return todo!("Do work and return a response");
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
    pub fn add_global_command(&mut self, name: &'static str, func: HandlerFunction) {
        self.global_handles.insert(name, func);
    }

    pub fn add_component_handle(&mut self, custom_id: &'static str, func: HandlerFunction){
        self.component_handles.insert(custom_id, func);
    }

    /// Entry point function for handling `Interactions`
    pub async fn interaction(&self, req: HttpRequest, body: String) -> Result<HttpResponse> {
        // Check for good content type --> must be application/json

        if let Some(ct) = req.headers().get("Content-Type") {
            if ct != "application/json" {
                debug!(
                    "Incoming interaction rejected, bad Content-Type specified. Origin: {:?}",
                    req.connection_info().realip_remote_addr()
                );
                return ERROR_RESPONSE!(400, "Bad Content-Type");
            }
        } else {
            debug!(
                "Incoming interaction rejected, no Content-Type specified. Origin: {:?}",
                req.connection_info().realip_remote_addr()
            );
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
                debug!(
                    "Incoming interaction rejected, invalid signature. Origin: {:?}",
                    req.connection_info().realip_remote_addr()
                );
                return ERROR_RESPONSE!(401, "Invalid request signature");
            }
        } else {
            // If proper headers are not present reject.
            debug!(
                "Incoming interaction rejected, missing headers. Origin: {:?}",
                req.connection_info().realip_remote_addr()
            );
            return ERROR_RESPONSE!(400, "Bad signature data");
        }

        // Security checks passed, try deserializing request to Interaction.
        match serde_json::from_str::<Interaction>(&body) {
            Err(e) => {
                // It's probably bad on our end if this code is reached.
                error!("Failed to decode interaction! Error: {}", e);
                debug!("Body sent: {}", body);
                return ERROR_RESPONSE!(400, format!("Bad body: {}", e));
            }
            Ok(interaction) => {
                match interaction.r#type {
                    InteractionType::Ping => {
                        let response =
                            InteractionResponse::new(InteractionResponseType::Pong, None);
                        debug!("Got a ping, responding with pong.");
                        return Ok(HttpResponse::build(StatusCode::OK)
                            .content_type("application/json")
                            .json(response));
                    }

                    InteractionType::ApplicationCommand => {
                        let data = if let Some(ref data) = interaction.data {
                            data
                        } else {
                            error!("Failed to unwrap Interaction!");
                            return ERROR_RESPONSE!(500, "Failed to unwrap");
                        };

                        if let Some(handler) =
                            self.global_handles.get(data.name.as_ref().unwrap().as_str())
                        {
                            // do stuff with v if needed

                            // construct a Context
                            let ctx = Context::new(self.client.clone(), interaction);

                            // Call the handler
                            let response = handler(ctx).await;

                            if response.r#type
                                == InteractionResponseType::DefferedChannelMessageWithSource
                            {
                                /* The use of HTTP code 202 is more appropriate when an Interaction is deffered.
                                If an application is first sending a deffered channel message response, this usually means the system
                                is still processing whatever it is doing.
                                See the spec: https://tools.ietf.org/html/rfc7231#section-6.3.3 */
                                Ok(HttpResponse::build(StatusCode::ACCEPTED).json(response))
                            } else {
                                // Send out a response to Discord
                                let r = HttpResponse::build(StatusCode::OK).json(response);

                                Ok(r)
                            }
                        } else {
                            error!(
                                "No associated handler found for {}",
                                data.name.as_ref().unwrap().as_str()
                            );
                            ERROR_RESPONSE!(500, "No associated handler found")
                        }
                    }
                    InteractionType::MessageComponent => {
                        let data = if let Some(ref data) = interaction.data {
                            data
                        } else {
                            error!("Failed to unwrap Interaction!");
                            return ERROR_RESPONSE!(500, "Failed to unwrap");
                        };

                        if let Some(handler) =
                            self.component_handles.get(data.custom_id.as_ref().unwrap().as_str())
                        {
                            // construct a Context
                            let ctx = Context::new(self.client.clone(), interaction);

                            // Call the handler
                            let response = handler(ctx).await;

                            if response.r#type
                                == InteractionResponseType::DefferedUpdateMessage
                            {
                                /* The use of HTTP code 202 is more appropriate when an Interaction is deffered.
                                If an application is first sending a deffered channel message response, this usually means the system
                                is still processing whatever it is doing.
                                See the spec: https://tools.ietf.org/html/rfc7231#section-6.3.3 */
                                Ok(HttpResponse::build(StatusCode::ACCEPTED).json(response))
                            } else {
                                // Send out a response to Discord
                                let r = HttpResponse::build(StatusCode::OK).json(response);

                                Ok(r)
                            }
                        } else {
                            error!(
                                "No associated handler found for {}",
                                data.name.as_ref().unwrap().as_str()
                            );
                            ERROR_RESPONSE!(500, "No associated handler found")
                        }
                    }
                    
                }
            }
        }
    }

    /// This is a predefined function that starts an `actix_web::HttpServer` and binds `self.interaction` to `/api/discord/interactions`.
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
