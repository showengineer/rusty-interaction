# Rusty Interaction
This library is intended to receive Discord interactions though your own API endpoint. 

**This is a Work-in-progress!!!**
The code is a straight rip out of one of my bots. It might not be very flexible at this stage.

## Install instructions
Due to the fact that this library is far from done, it hasn't been published to the crates.io repo (yet). However, installing the library isn't very hard.
Follow the following steps:
1. Clone this repository. I'd recommend adding this to your project using git's submodules: `git submodule add https://github.com/hugopilot/rusty-interaction`
1. Go to your `Cargo.toml` file. Add the following to this file:
```toml
[dependencies.rusty_interaction]
path = "../rusty-interaction"
```
`path` is the location of the repo you just cloned. The library comes with the `types` and `security` features by default. If you wish to use the `handler` feature, add `features = ["handler"]` to the dependency entry.
1. Profit!

## Contributing
More than welcome! :D

## What it has right now
- [x] - Type exposure
- [x] - Interaction validation
- [x] - Receive Interactions from Discord
- [x] - Bind interactions to a function (with the help of a macro)
- [ ] - Properly respond to interactions from Discord
- [ ] - Nice system to make follow-up messages.
- [ ] - Not a pile of code spaghetti that just works (oops...👀)


## Difference between receiving interactions through the gateway and your own endpoint
The gateway requires you to have a discord client where you receive interactions. 
Setting up your own endpoint makes Discord send the interactions to your own API endpoint (ex. `https://example.com/api/discord/interactions`).

If you already have an API that runs your service and you're looking to integrate with Discord, this way of receiving interactions is really interesting.

### Ok, I want to receive interactions through the gateway. Does your library support that?
No. If you want to receive interactions through the gateway [Serenity](https://github.com/serenity-rs/serenity) is your friend.
