# Example 7: Embeds

Embeds are a way of displaying rich content in Discord messages.

This is what this demo will produce:

![demo gif](https://user-images.githubusercontent.com/10338882/121349853-45b17800-c92a-11eb-80d6-d182b685c468.gif)



# Running this example
You can use regular `cargo build` and `cargo run` commands.

To run this example:

`cargo run`. Note that you'll need to edit the `PUB_KEY` constant accordingly (it will panic if you don't give a vaild key).
You'll also need to supply a TLS certificate and it's corresponding private key (`cert.pem` and `key.pem` by default).

# Useful documentation
- [`types::embed` module](https://docs.rs/rusty_interaction/latest/rusty_interaction/types/embed/index.html)
