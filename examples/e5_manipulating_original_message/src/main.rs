#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::InteractionHandler;
use rusty_interaction::types::interaction::*;

// Import for using components
use rusty_interaction::types::components::*;


use std::time::Duration;
use async_std::task;

use rusty_interaction::actix::Arbiter;

const PUB_KEY: &str = "YOUR_PUBLIC_KEY"; 

const APP_ID: u64 = 0; 

#[slash_command]
async fn test(ctx: Context) -> Result<InteractionResponse, ()>{


    let m = ctx.clone();
    // Spawn a new thread before sending a response. 
    Arbiter::spawn(async move {

        // Wait three seconds and delete
        task::sleep(Duration::from_secs(3)).await;

        m.delete_original().await;
    });
    

    return ctx.respond()
        .message("I was summoned?")
        .finish();
}

// The lib uses actix-web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    
    let mut handle = InteractionHandler::new(APP_ID, PUB_KEY, None);
    
    
    handle.add_global_command("summon", test);


    return handle.run(10043).await;
    
}

