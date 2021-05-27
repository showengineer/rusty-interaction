#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::InteractionHandler;
use rusty_interaction::types::interaction::*;

// Used for getting TLS to work
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

// This key is needed for verifying incoming Interactions. This verification is mandatory. 
// You can find this key in the Discord Developer Portal. 
const PUB_KEY: &str = "YOUR_APP'S_PUBLIC_KEY"; 


// This macro will transform the function to something the handler can use
#[slash_command]
// Function handlers should take an `Interaction` object and should return an `InteractionResponse`
async fn test(ctx: Context) -> InteractionResponse{
    println!("I HAVE BEEN SUMMONED!!!");
        
    // Return a response by using the `Context.respond` function.
    // `Context.respond` returns an `InteractionResponseBuilder`.
    // You can now build a `InteractionResponse` by using it's functions.
    return ctx.respond()
            .message("I was summoned?")
            .finish();
}

// The lib uses actix-web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initalize our InteractionHandler
    // This will handle incoming interactions and route them to your own handlers
    let mut handle = InteractionHandler::new(PUB_KEY);
    
    // This will tell the handler to route the `/summon` command to the test function. So if someone uses `/summon`, test() will be called.
    // Please note that you'll need to register your commands to Discord if you haven't yet. This library only handles incoming Interactions (as of now),
    // not command management.
    handle.add_global_command("summon", test);

    // This is to setup TLS. 
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    // Run the API. Note the use of run_ssl(config) instead of run()
    // The server runs on port 10443!
    return handle.run_ssl(config).await;
    
}

