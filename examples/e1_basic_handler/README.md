# Example 1: A basic handler
This the most basic handler you can make with Rusty Interaction. 

If `/summon` was called, it will print `I HAVE BEEN SUMMONED!!!` on the console and reply with `I was summoned?`.

# Running this example
You can use regular `cargo build` and `cargo run` commands.

To run this example:
`cargo run`. Note that you'll need to edit the `PUB_KEY` constant accordingly (it will panic if you don't give a vaild key).

# Security
This example starts a plain HTTP server. Using plain HTTP these days is a **bad idea**. 

Look at example 2 for a HTTPS server implementation.