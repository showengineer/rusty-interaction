# Rusty Interaction
This library provides types and helper functions for handling Discord's [Interactions](https://discord.com/developers/docs/interactions/slash-commands). Optionally, it can provide an actix-web backend handler system to handle Interactions through your own API (instead of using the gateway).

**This library is not done. Use for production not yet recommended**

Although it's usable and supports most of the features, it still requires work and things are subject to change. 
## Install instructions
Add this dependency entry to your `cargo.toml` file:
```toml
rusty-interaction = "0"
```

## Contributing
More than welcome! :D

## What it has right now
- [x] - Data models exposure
- [x] - Interaction validation (`crate::security::verify_discord_message()`)
- [x] - Receive Interactions from Discord
- [x] - Bind interactions to a function (with the help of a macro)
- [x] - Properly respond to interactions from Discord
- [x] - Nice system to make follow-up messages.
- [ ] - Nice system to manage guild-specific commands.
- [ ] - Not a pile of code spaghetti that just works (oops...👀)


## Difference between receiving interactions through the gateway and your own endpoint
The gateway requires you to have a discord client where you receive interactions. 
Setting up your own endpoint makes Discord send the interactions to your own API endpoint (ex. `https://example.com/api/discord/interactions`).

If you already have an API that runs your service and you're looking to integrate with Discord, this way of receiving interactions can be really interesting.

### Ok, I want to receive interactions through the gateway. Does your library support that?
No. If you want to receive interactions through the gateway, you want to take a look at [Serenity](https://github.com/serenity-rs/serenity) or one of the [other libraries](https://discord.com/developers/docs/topics/community-resources#libraries-discord-libraries).
