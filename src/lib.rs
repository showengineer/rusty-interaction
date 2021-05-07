#![warn(missing_docs)]
//! Rusty-interaction is a library that allows you to work with Discord's new [Interactions](https://blog.discord.com/slash-commands-are-here-8db0a385d9e6).
//! It can expose types and provides helper functions to validate your Interactions.
//! It can optionally provide a handler that allows you to receive interactions via outgoing webhook.
//! 
//! Note that Discord also has [official documentation](https://discord.com/developers/docs/intro).
//! 
//! ## Examples
//! See the [`examples`](https://github.com/hugopilot/rusty-interaction/tree/main/examples`) directory.

#[macro_use]
mod macros;

#[allow(dead_code)]
const BASE_URL: &str = "https://discord.com/api/v9";

#[cfg(feature = "types")]
/// Exposes useful data models
pub mod types;

/// Provides a helper function to validate Discord interactions.
#[cfg(feature = "security")]
pub mod security;

/// Provides an entire handler to handle Discord interactions.
#[cfg(feature = "handler")]
pub mod handler;
#[cfg(feature = "handler")]
pub use actix;
#[cfg(feature = "handler")]
pub use attributes::*;
#[cfg(feature = "handler")]
pub use actix_web;

#[cfg(all(test, not(feature = "handler")))]
compile_error!(
    "cannot run tests without the 'handler' feature enabled (run with --features handler)"
);

#[cfg(all(test, feature = "handler"))]
mod tests;

// ===== USEFUL MACROS =====
#[macro_export]
#[doc(hidden)]
macro_rules! expect_successful_api_response {
    ($response:ident, $succret:expr) => {
        match $response {
            Err(e) => {
                debug!("Discord API request failed: {:#?}", e);
                Err(HttpError {
                    code: 0,
                    message: format!("{:#?}", e),
                })
            }
            Ok(r) => {
                let st = r.status();
                if !st.is_success() {
                    let e = format!("{:#?}", r.text().await);
                    debug!("Discord API returned an error: {:#?}", e);
                    Err(HttpError {
                        code: st.as_u16(),
                        message: e,
                    })
                } else {
                    $succret
                }
            }
        }
    };
}
#[macro_export]
#[doc(hidden)]
macro_rules! expect_specific_api_response {
    ($response:ident, $expres:expr, $succret:expr) => {
        match $response {
            Err(e) => {
                debug!("Discord API request failed: {:#?}", e);

                Err(HttpError {
                    code: 0,
                    message: format!("{:#?}", e),
                })
            }
            Ok(r) => {
                let st = r.status();
                if st != $expres {
                    let e = format!("{:#?}", r.text().await);
                    debug!("Discord API returned an error: {:#?}", e);
                    Err(HttpError {
                        code: st.as_u16(),
                        message: e,
                    })
                } else {
                    $succret
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! expect_successful_api_response_and_return {
    ($response:ident, $struc:ident, $retval:ident, $succret:expr) => {
        match $response {
            Err(e) => {
                debug!("Discord API request failed: {:#?}", e);
                Err(HttpError {
                    code: 0,
                    message: format!("{:#?}", e),
                })
            }
            Ok(r) => {
                let st = r.status();
                if !st.is_success() {
                    let e = format!("{:#?}", r.text().await);
                    debug!("Discord API returned an error: {:#?}", e);
                    Err(HttpError {
                        code: st.as_u16(),
                        message: e,
                    })
                 } else {
                    let a: Result<$struc, serde_json::Error> =
                        serde_json::from_str(&r.text().await.unwrap());

                    match a {
                        Err(e) => {
                            debug!("Failed to decode response: {:#?}", e);
                            Err(HttpError {
                                code: 500,
                                message: format!("{:?}", e),
                             })
                        }
                        Ok($retval) => {
                            $succret
                        }
                    }
                }
            }
        }
    };
}
