# Example 8: Managing commands

From version 0.1.7, you can access the `InteractionHandler` that you can use to, for example, create or delete guild-specific commands.

When using this demo, `/summon` will create a `/generated` command. If you use `/generated`, it will delete/unregister itself.

Note: These features are included in the `extended-handler` feature!

# Important design note
The handler does not 'remember' what guild-specific commands are registered and to which function they were attached.

There are plans to make the handler 'remember' this.

# Running this example
You can use regular `cargo build` and `cargo run` commands.

To run this example:

`cargo run`. Note that you'll need to edit the `PUB_KEY` constant accordingly (it will panic if you don't give a vaild key).
You'll also need to supply a TLS certificate and it's corresponding private key (`cert.pem` and `key.pem` by default).

# Useful documentation
- [InteractionHandler](https://docs.rs/rusty_interaction/latest/rusty_interaction/handler/struct.InteractionHandler.html)
