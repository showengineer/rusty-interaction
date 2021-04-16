pub mod handler;
pub mod security;
pub mod types;

#[macro_use]
pub mod macros;


#[cfg(test)]
mod tests;


use crate::security::*;
use crate::types::*;

use log::{debug, error, log_enabled, info, Level};

use actix_web::http::{StatusCode};
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result,};
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use ed25519_dalek::{PublicKey};
use hex;

use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};

use std::collections::HashMap;
use std::boxed::Box;
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;


pub trait FromInteraction : Sized{
    type Future: Future<Output = ()>;
    fn from_interaction(interaction: &Interaction) -> Self::Future;
}

pub trait Factory<T, R>: Clone + 'static
where
    R: Future<Output = ()>,
{
    fn call(&self, param: T) -> R;
}


pub trait Handler<T, R>: Clone + 'static
where
    R: Future<Output = ()>
{
    fn call(&self, param: T) -> R;
}

pub type HandlerResponse = Option<InteractionResponse>;

#[derive(Clone)]
pub struct InteractionHandler {
    app_public_key: PublicKey,
    //handles: HashMap::<&'static str, fn(&'_ Interaction) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>>,
    handles: HashMap::<&'static str, fn(&'_ Interaction) -> Pin<Box<dyn Future<Output = HandlerResponse> + Send + '_>>>,
}


impl InteractionHandler {
    pub fn new(pbk_str: &str) -> InteractionHandler {
        let bytes = hex::decode(pbk_str);

        // Init a normal array.
        let mut pbk_bytes: [u8; PUBLIC_KEY_LENGTH] = [0;PUBLIC_KEY_LENGTH];

        match bytes{
            Err(_) => panic!("Failed to parse the public key"),
            Ok(k) => {
                if k.len() != PUBLIC_KEY_LENGTH{
                    panic!("Failed to parse the public key (bad length)");
                }
                pbk_bytes = security::convert_to_arr::<u8, PUBLIC_KEY_LENGTH>(k);
            }
        };
        let pbk = PublicKey::from_bytes(&pbk_bytes);
        if pbk.is_err() {
            panic!("Failed to convert public key.");
        }
        return InteractionHandler {
            app_public_key: pbk.unwrap(),
            handles: HashMap::new(),
        };
    }

    pub fn add_command(&mut self, name: &'static str, func: fn(&'_ Interaction) -> Pin<Box<dyn Future<Output = HandlerResponse> + Send + '_>>){
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
        match serde_json::from_slice::<Interaction>(&body){
            Err(e) => {
                error!("BAD FORM: {:?}. Error: {}", body, e);
                ERROR_RESPONSE!(400, format!("Bad body: {}", e));
                
            },
            Ok(v) =>{
                //TODO: Reponds with OK PONG, parse it to interaction handler
                if v.r#type == InteractionType::PING{
                    let response = InteractionResponse::new(InteractionResponseType::PONG, None);
                    info!("RESP: PONG");
                    return Ok(HttpResponse::build(StatusCode::OK)
                        .content_type("application/json")
                        .json(response));

                    
                }
                else{
                    if v.data.is_none(){
                        ERROR_RESPONSE!(500, "Failed to unwrap");
                    }

                    let dat = v.clone().data.unwrap();


                    match self.handles.get(dat.name.as_str()){
                        Some(f) =>{
                            // Call the handler
                            let r = f(&v).await;

                            // Send out a response to Discord
                            match r {
                                None => {return Ok(HttpResponse::build(StatusCode::OK).finish());},
                                Some(resp) => { return Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(resp));}
                            }
                        },
                        None => {ERROR_RESPONSE!(500, "No associated handler found");}
                    }
                   
                }
                
            }
        }
        
    }

    /// This is a predefined function that starts an `actix_web::HttpServer` and binds `self.interaction` to `/api/discord/interacitons`. 
    /// Note that you'll eventually have to switch to an HTTPS server. This function does not provide this.
    pub async fn run(self) -> std::io::Result<()>{
        HttpServer::new(move || {
            App::new()
            .data(self.clone())
            .route("/api/discord/interactions", web::post().to(|data: web::Data<InteractionHandler>, req: HttpRequest, body: web::Bytes|{ 
                let data = data.into_inner();
                async move{
                    (*data).clone().interaction(req, body).await
                }
            }))
        })
        .bind("0.0.0.0:10080")?
        .run()
        .await
    }

    pub async fn run_ssl(self, server_conf: ServerConfig) -> std::io::Result<()>{
        HttpServer::new(move || {
            App::new()
            .data(self.clone())
            .route("/api/discord/interactions", web::post().to(|data: web::Data<InteractionHandler>, req: HttpRequest, body: web::Bytes|{ 
                let data = data.into_inner();
                async move{
                    (*data).clone().interaction(req, body).await
                }
            }))
        })
        .bind_rustls("0.0.0.0:10443", server_conf)?
        .run()
        .await
    }


   

}

/// Simpler header getter from a HTTP request
fn get_header<'a>(req: &'a HttpRequest, header: &str) -> Option<&'a str>{
    req.headers().get(header)?.to_str().ok()
}



