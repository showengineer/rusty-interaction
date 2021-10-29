#[macro_use]
extern crate rusty_interaction;

use rusty_interaction::handler::{InteractionHandler, ManipulationScope};
use rusty_interaction::types::interaction::*;
// Relevant imports here
use rusty_interaction::types::application::{
    ApplicationCommandOption, ApplicationCommandOptionType, SlashCommandDefinitionBuilder,
};
use rusty_interaction::Builder;

const PUB_KEY: &str = "YOUR PUB KEY";
const APP_ID: u64 = 000000000000000000;

// Must implement Clone
#[derive(Clone)]
struct MyStruct {
    pub foo: u16,
}

#[slash_command]
async fn test(
    handler: &mut InteractionHandler,
    ctx: Context,
) -> Result<InteractionResponse, std::convert::Infallible> {
    // Get a mutable reference to MyStruct
    let my_struct = handler.data.get_mut::<MyStruct>().unwrap();

    my_struct.foo += 1;

    return ctx
        .respond()
        .content(format!("Foo is {}", my_struct.foo))
        .finish();
}

// The lib uses actix-web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let my_struct = MyStruct { foo: 0 };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut handle = InteractionHandler::new(APP_ID, PUB_KEY, None);

    handle.add_global_command("summon", test);

    // Add my_struct to the Data map
    handle.add_data(my_struct);

    return handle.run(10080).await;
}
