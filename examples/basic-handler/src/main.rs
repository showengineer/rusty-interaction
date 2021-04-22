#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::{InteractionHandler};
use rusty_interaction::types::*;


// This key is needed for verifying incoming Interactions. This verification is mandatory. 
// You can find this key in the Discord Developer Portal. 
const PUB_KEY: &str = "YOUR_APP'S_PUBLIC_KEY"; 


// This macro will transform the function to something the handler can use
// I am planning on making this an attribute macro in the future.
SLASH_COMMAND!{

    // Function handlers should take an `Interaction` object and should return an `InteractionResponse`
    async fn test(interaction: &'interaction Interaction) -> InteractionResponse{
        println!("I HAVE BEEN SUMMONED!!!");
        
        // Return a response by using the `Interaction.response` function.
        // `Interaction.response()` returns an `InteractionResponse` without any data
        // You can now manipulate this object by using it's functions.
        return interaction.response(InteractionResponseType::CHANNEL_MESSAGE_WITH_SOURCE)
                .message(&String::from("Oh hey there :D!"))
                // Finish returns self and takes ownership over the object. 
                .finish();
    }
}

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
    handle.add_command("summon", test);

    // Run the API server!
    return handle.run(config).await;
    
}