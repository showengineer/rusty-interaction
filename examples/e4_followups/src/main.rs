#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::InteractionHandler;
use rusty_interaction::types::interaction::*;

use std::time::Duration;
use async_std::task;


const PUB_KEY: &str = "YOUR_PUBLIC_KEY"; 
const APP_ID: u64 = 0; 

#[slash_command]
async fn test(ctx: Context) -> Result<InteractionResponse, ()>{
    

    // Send a followup message
    let fu = ctx.clone().create_followup(&
        WebhookMessage::default().content("This is a follow up!")
    ).await;
    
    
    // Mind you: The return value is the INITIAL RESPONSE. What is returned here is sent directly to Discord
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

    return handle.run(10443).await;
    
}

