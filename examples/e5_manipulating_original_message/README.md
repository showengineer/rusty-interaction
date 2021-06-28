# Example 5: Manipulating original messages

This basic example shows how to delete a message three seconds after it has been sent.

This example spawns a new thread that waits for three seconds before deleting and then sends the message.

# Running this example
You can use regular `cargo build` and `cargo run` commands.

To run this example:

`cargo run`. Note that you'll need to edit the `PUB_KEY` constant accordingly (it will panic if you don't give a vaild key).
