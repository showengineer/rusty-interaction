# Example 4: Creating Follow-up messages

Sometimes, responding with just one message isn't enough. For those situations we have 'follow-up' messages.

These are webhook messages and are just like other messages (except you can add linked text too!).

Just like InteractionResponses, you can edit and delete as you wish.

# Running this example
You can use regular `cargo build` and `cargo run` commands.

To run this example:

`cargo run`. Note that you'll need to edit the `PUB_KEY` constant accordingly (it will panic if you don't give a vaild key).

# Useful documentation
- [FollowupMessage](https://docs.rs/rusty_interaction/latest/rusty_interaction/types/interaction/struct.FollowupMessage.html)
