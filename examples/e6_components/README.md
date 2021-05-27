# Example 6: Components

Components are an exciting way of user interaction with your bot. 

With components, you can add rich elements like buttons to your messages!

Example: (add pic)

# Running this example
You can use regular `cargo build` and `cargo run` commands.

To run this example:

`cargo run`. Note that you'll need to edit the `PUB_KEY` constant accordingly (it will panic if you don't give a vaild key).
You'll also need to supply a TLS certificate and it's corresponding private key (`cert.pem` and `key.pem` by default).

# Useful documentation
- [add_component_handler](https://docs.rs/rusty_interaction/latest/rusty_interaction/handler/struct.InteractionHandler.html#method.add_component_handle) 
- [component_handler proc macro](https://docs.rs/rusty_interaction/latest/rusty_interaction/attr.component_handler.html)
- [types::components module](https://docs.rs/rusty_interaction/latest/rusty_interaction/types/components/index.html)