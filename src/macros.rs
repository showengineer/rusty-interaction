/// Macro that generates an `HttpResponse` containing a message serialized in JSON
#[macro_export]
#[doc(hidden)]
macro_rules! ERROR_RESPONSE {
    ($status:expr, $message:expr) => {{
        let emsg = $crate::types::MessageError::new(::std::string::String::from($message));

        Ok(::actix_web::HttpResponse::build(
            ::actix_web::http::StatusCode::from_u16($status).unwrap(),
        )
        .json(emsg))
    }};
}
