# Rusty Interaction
This library provides types and helper functions for handling Discord's [Interactions](https://discord.com/developers/docs/interactions/slash-commands). Optionally, it can provide an actix-web backend handler system to handle Interactions through your own API (instead of using the gateway).

**This is a Work-in-progress!!!**
The code is a straight rip out of one of my bots and it needs work.

## Install instructions
Due to the fact that this library is far from done, it hasn't been published to the crates.io repo (yet). However, installing the library is just as easy as normal.
Follow the following steps:
Add this dependency entry to your `cargo.toml` file:
```toml
rusty-interaction = { git = "https://github.com/hugopilot/rusty-interaction" } 
```
`path` is the location of the repo you just cloned, change it accordingly. The library comes with the `types` and `security` features by default. If you also wish to use the `handler` feature, add `features = ["handler"]` to the dependency entry.


## Contributing
More than welcome! :D

## What it has right now
- [x] - Data models exposure
- [x] - Interaction validation (`crate::security::verify_discord_message()`)
- [x] - Receive Interactions from Discord
- [x] - Bind interactions to a function (with the help of a macro)
- [x] - Properly respond to interactions from Discord
- [ ] - Nice system to make follow-up messages.
- [ ] - Not a pile of code spaghetti that just works (oops...👀)


## Difference between receiving interactions through the gateway and your own endpoint
The gateway requires you to have a discord client where you receive interactions. 
Setting up your own endpoint makes Discord send the interactions to your own API endpoint (ex. `https://example.com/api/discord/interactions`).

If you already have an API that runs your service and you're looking to integrate with Discord, this way of receiving interactions can be really interesting.

### Ok, I want to receive interactions through the gateway. Does your library support that?
No. If you want to receive interactions through the gateway, you want to take a look at [Serenity](https://github.com/serenity-rs/serenity) or one of the [other libraries](.
