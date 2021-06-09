#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::InteractionHandler;
use rusty_interaction::types::interaction::*;

// Relevant imports here
use rusty_interaction::types::application::{SlashCommandDefinitionBuilder};



// Used for getting TLS to work
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

const PUB_KEY: &str = "Your public key"; 
const TOKEN: &str = "Some Token";
const APP_ID: u64 = 000000000000000000;

#[slash_command]
async fn delete_self(handler: &mut InteractionHandler, ctx: Context) -> InteractionResponse{
    let sec_ctx = ctx.clone();
    if let Some(g) = sec_ctx.interaction.guild_id{
        if let Some(data) = sec_ctx.interaction.data{
            let cid = data.id;

            // Using this to remove the guild command
            let r = handler.deregister_command_handle(g, cid.unwrap()).await;
            if r.is_ok(){
                return ctx.respond().content("`/generated` deleted!").finish();
            }
            else{
                return ctx.respond().content("Something went wrong!").finish();
            }
        }
        return ctx.respond().content("Something went wrong!").finish();
    }
    else{
        return ctx.respond().content("This command should be invoked in a guild!").finish();
    }
}

#[slash_command]
async fn test(handler: &mut InteractionHandler, ctx: Context) -> InteractionResponse{

    if let Some(i) = ctx.interaction.guild_id{
        
        // Build a simple command
        let cmd = SlashCommandDefinitionBuilder::default()
                    .name("generated")
                    .description("This is a generated guild command!")
                    .finish();

        // Register that command
        if let Ok(_) = handler.register_command_handle(i, cmd, delete_self).await{
            return ctx.respond().content("`/generated` has been registered!").finish();
        }
        else{
            return ctx.respond().content("Something went wrong!").finish();
        }
        
    }
    else{
        return ctx.respond().content("Not in a guild!").finish();
    }
}



// The lib uses actix-web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Note the use of the TOKEN. You'll also need to specify your application ID
    let mut handle = InteractionHandler::new(APP_ID, PUB_KEY, Some(TOKEN));
    
    
    handle.add_global_command("summon", test);

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    return handle.run_ssl(config, 10443).await;
    
}
