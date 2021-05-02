#[cfg(feature="security")]
pub mod security;

#[cfg(feature="types")]
pub mod types;
#[cfg(feature="handler")]
pub mod handler;


pub use attributes::*;

pub mod macros;

#[cfg(test)]
mod tests;

const BASE_URL: &str = "https://discord.com/api/v9";


#[macro_export]
macro_rules! SLASH_COMMAND {(
    $( #[$attr:meta] )* // includes doc strings
    $pub:vis
    async
    fn $fname:ident ( $($args:tt)* ) $(-> $Ret:ty)?
    {
        $($body:tt)*
    }
) => (
    $( #[$attr] )*
    #[allow(unused_parens)]
    $pub
    fn $fname<'context> ( $($args)* ) -> ::std::pin::Pin<::std::boxed::Box<
        dyn 'context + Send + ::std::future::Future<Output = ($($Ret)?)>
    >>
    {
        Box::pin(async move { $($body)* })
    }
)}




