# Example 2: Another basic handler, but with TLS
Practically the same as example 1, but with TLS integrated.


# Running this example
You can use regular `cargo build` and `cargo run` commands.

To run this example:

`cargo run`. 

Note that you'll need to edit the `PUB_KEY` constant accordingly (it will panic if you don't give a vaild key).
You'll also need to supply a TLS certificate and it's corresponding private key (`cert.pem` and `key.pem` by default).

# Docs to read
- [actix tls example](https://github.com/actix/examples/tree/master/security/rustls)