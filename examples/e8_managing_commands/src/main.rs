#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::{InteractionHandler, ManipulationScope};
use rusty_interaction::types::interaction::*;
// Relevant imports here
use rusty_interaction::types::application::{SlashCommandDefinitionBuilder, ApplicationCommandOption, ApplicationCommandOptionType};

const PUB_KEY: &str = "MY PUB KEY"; 
const TOKEN: &str = "MY TOKEN";
const APP_ID: u64 = 00000000000000000;

#[slash_command]
async fn delete_self(handler: &mut InteractionHandler, ctx: Context) -> InteractionResponse{
    let sec_ctx = ctx.clone();
    if let Some(g) = sec_ctx.interaction.guild_id{
        if let Some(data) = sec_ctx.interaction.data{
            let cid = data.id;

            // Using this to remove the guild command
            let r = handler.deregister_guild_handle(g, cid.unwrap(), &ManipulationScope::All).await;
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
                    .add_option(
                        ApplicationCommandOption::default()
                        .option_type(&ApplicationCommandOptionType::String)
                        .name("string")
                        .description("I will do absolutely nothing with this")
                    )
                    .finish();

        match handler.register_guild_handle(i, &cmd, delete_self, &ManipulationScope::All).await{
            Ok(_) => {
                return ctx.respond().content("`/generated` has been registered!").finish();
            }
            Err(e) => {
                return ctx.respond().content(format!("Error ({}): \n```json\n{:?}```", e.code, e.message)).finish();
            }
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
    let mut handle = InteractionHandler::new(APP_ID, PUB_KEY, Some(&TOKEN.to_string()));
    
    
    handle.add_global_command("summon", test);


    return handle.run(10443).await;
    
}

