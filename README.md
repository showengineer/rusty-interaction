# Discord Interactions
This library is intended to receive Discord Interactions though your own API endpoint. 

**This is a Work-in-progress!!!**
The code is a straight rip out of one of my bots. It might not be very flexible at this stage.

## What it has right now
- [x] - Receive Interactions from Discord
- [x] - Bind interactions to a function (with the help of a macro)
- [ ] - Properly respond to interactions from Discord
- [ ] - Nice system to make follow-up messages.
- [ ] - Not a pile of code spaghetti that just works (oops...👀)


## Difference between receiving interactions through the gateway and your own endpoint
The gateway requires you to have a discord client where you receive interactions. 
Setting up your own endpoint makes Discord send the interactions to your own API endpoint (ex. `https://example.com/api/discord/interactions`).

If you already have an API that runs your service and you're looking to integrate with Discord, this way of receiving interactions is really interesting.

### Ok, I want to receive interactions through the gateway. Does your library support that.
No. If you want to receive interactions through the gateway [Serenity](https://github.com/serenity-rs/serenity) is your friend.
