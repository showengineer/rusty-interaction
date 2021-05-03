/// Macro that generates an `HttpResponse` containing a message serialized in JSON
#[macro_export]
macro_rules! ERROR_RESPONSE {
    ($status:expr, $message:expr) => {
        let emsg = MessageError::new(String::from($message));

        return Ok(HttpResponse::build(StatusCode::from_u16($status).unwrap())
            .content_type("application/json")
            .json(emsg));
    };
}
