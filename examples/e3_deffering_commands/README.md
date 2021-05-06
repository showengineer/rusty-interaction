# Example 3: Deffered commands
Discord wants you to reply to a slash command in no more than three seconds. However, some tasks simply take longer to complete than three seconds.

To make this possible, you can send a 'deffered command response'. If you do that, the user will be notified that the bot is processing their request.

You'll still have to reply in _no more than 15 minutes_, or the interaction will expire.

To indicate you want to reply with a deffered response first, simply put `#[defer]` under the `#[slash_command]` proc-macro! The way you respond to an interaction stays the same: the compiler transforms this into the correct form for you. Example:

```rs
#[slash_command]
#[defer]
async fn some_handler(ctx: Context) -> InteractionResponse{
    // ...

    return ctx.respond().content("Wowh! That was quite a task!");
}
```

# Running this example
You can use regular `cargo build` and `cargo run` commands.

To run this example:

`cargo run`. Note that you'll need to edit the `PUB_KEY` constant accordingly (it will panic if you don't give a vaild key).
You'll also need to supply a TLS certificate and it's corresponding private key (`cert.pem` and `key.pem` by default).