

/// Macro that generates an `HttpResponse` containing a message serialized in JSON
macro_rules! ERROR_RESPONSE {
    ($status:expr, $message:expr) => {
        let emsg = MessageError::new(String::from($message));
        
        return Ok(HttpResponse::build(StatusCode::from_u16($status).unwrap())
            .content_type("application/json")
            .json(emsg));
    };
}

#[macro_export]
macro_rules! SLASH_COMMAND {(
    $( #[$attr:meta] )* // includes doc strings
    $pub:vis
    async
    fn $fname:ident<$lt:lifetime> ( $($args:tt)* ) $(-> $Ret:ty)?
    {
        $($body:tt)*
    }
) => (
    $( #[$attr] )*
    #[allow(unused_parens)]
    $pub
    fn $fname<$lt> ( $($args)* ) -> ::std::pin::Pin<::std::boxed::Box<
        dyn $lt + Send + ::std::future::Future<Output = ($($Ret)?)>
    >>
    {
        Box::pin(async move { $($body)* })
    }
)}
