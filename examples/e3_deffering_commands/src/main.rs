#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::InteractionHandler;
use rusty_interaction::types::interaction::*;

use std::time::Duration;
use async_std::task;

const PUB_KEY: &str = "YOUR_APP'S_PUBLIC_KEY"; 

const APP_ID: u64 = 0; 

#[slash_command]
// Sending a deffered response by adding the `#[defer]` attribute
#[defer]
async fn test(ctx: Context) -> Result<InteractionResponse, ()>{
    println!("I HAVE BEEN SUMMONED!!!");
    
    // This is representing some work that needs to be done before a response can be made
    task::sleep(Duration::from_secs(5)).await;
    
    return ctx.respond()
        .message("I was summoned?")
        .finish();
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    
    let mut handle = InteractionHandler::new(APP_ID, PUB_KEY, None);
    
    handle.add_global_command("summon", test);


    return handle.run(10443).await;
    
}

