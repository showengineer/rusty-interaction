use crate::handler::*;
use crate::security::*;
use crate::types;

use actix_rt;
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use ed25519_dalek::{PublicKey};
use std::sync::Mutex;

use actix_web::{test, HttpRequest, HttpResponse, HttpMessage, web, App};
use actix_web::http::{header, StatusCode};

const PUB_KEY: [u8; PUBLIC_KEY_LENGTH] = [
    0x82, 0xd8, 0xd9, 0x7f, 0xe0, 0x64, 0x1e, 0x68, 0xa1, 0xb0, 0xb1, 0x12, 0x20, 0xf0, 0x5e, 0x9e,
    0xa0, 0x53, 0x9a, 0xc, 0xdc, 0x0, 0x21, 0x19, 0xd4, 0xa9, 0xe9, 0xe0, 0x25, 0xab, 0xa1, 0xe9,
];

/*------------------------------
SECURITY TESTS
*/
#[test]
// Discord interaction verification test OK 1
fn crypto_verify_test_ok(){
    let pbk = PublicKey::from_bytes(&PUB_KEY);
    if pbk.is_err() {
        panic!("Failed to convert public key.");
    }
    let res = verify_discord_message(pbk.unwrap(), 
        "c41278a0cf22bf8f3061756063cd7ef548a3df23d0ffc5496209aa0ad4d9593343801bf11e099f41bca1afcac2c70734eebafede3dec7aac1caa5d8fade5af0c",
        "1616343571",
        &String::from("{\"type\" : 1}"));
    match res{
        Err(e) => {
            match e {
                ValidationError::KeyConversionError {name} => panic!("One of the keys failed to convert to proper types! Key: {}", name),
                ValidationError::InvalidSignatureError => panic!("Unexpected invalidation of signature"),
            }
        },
        Ok(_) =>{
            // Good!
        }
    }
}

#[test]
#[should_panic]
// Discord interacton verification test invalid 1
fn crypto_verify_test_fail(){
    let pbk = PublicKey::from_bytes(&PUB_KEY);
    if pbk.is_err() {
        panic!("Failed to convert public key.");
    }
    let res = verify_discord_message(pbk.unwrap(), 
        "69696969696969696696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696696969",
        "1616343571",
        &String::from("{\"type\" : 1}"));
    match res{
        Err(e) => {
            match e {
                ValidationError::KeyConversionError {name} => panic!("One of the keys failed to convert to proper types! Key: {}", name),
                ValidationError::InvalidSignatureError => panic!("Unexpected invalidation of signature"), // This is what it should be!
            }
        },
        Ok(_) =>{
            // Good!
        }
    }
}
/*-------------------------------
Discord Interactions API tests (endpoint: /api/discord/interactions)
*/

macro_rules! interaction_app_init {
    ($ih: ident) => {
        
        test::init_service(App::new()
        .data($ih.clone())
        .route("/api/discord/interactions", web::post().to(|data: web::Data<InteractionHandler>, req: HttpRequest, body: web::Bytes| async move{
            data.interaction(req, body).await
        })))
        .await;
    };
}

#[actix_rt::test]
// Request with bad content with no Content-Type header present
// Expected result: Return 400 without panicking
async fn interactions_no_content_type_header_test(){
    let ih = InteractionHandler::new(PUB_KEY);

    let mut app = interaction_app_init!(ih);

    let req = test::TestRequest::post()
        .uri("/api/discord/interactions")
        .set_payload("This is some malformed text { the system : can't really handle }")
        .to_request();

    let res: types::MessageError = test::read_response_json(&mut app, req).await;

    assert_eq!(res.message, "Bad Content-Type");

}

#[actix_rt::test]
// Request with bad content with no Content-Type header present
// Expected result: Return 400 without panicking
async fn interactions_bad_content_type_header_test(){
    let ih = InteractionHandler::new(PUB_KEY);

    let mut app = interaction_app_init!(ih);

    let req = test::TestRequest::post()
        .uri("/api/discord/interactions")
        .header("Content-Type", "plain/text")
        .set_payload("This is some malformed text { the system : can't really handle }")
        .to_request();

    let res: types::MessageError = test::read_response_json(&mut app, req).await;

    assert_eq!(res.message, "Bad Content-Type");

}

#[actix_rt::test]
// Request with missing X-Signature-Ed25519 Header
// Expected result: Return 400 without panicking
async fn interactions_no_signature_header_test(){
    let ih = InteractionHandler::new(PUB_KEY);

    let mut app = interaction_app_init!(ih);

    let req = test::TestRequest::post()
        .uri("/api/discord/interactions")
        .header("Content-Type", "application/json")
        .header("X-Signature-Timestamp", "1229349")
        .set_payload("This is some malformed text { the system : can't really handle }")
        .to_request();

    let res: types::MessageError = test::read_response_json(&mut app, req).await;

    assert_eq!(res.message, "Bad signature data");

}

#[actix_rt::test]
// Request with missing X-Signature-Timestamp Header
// Expected result: Return 400 without panicking
async fn interactions_no_timestamp_header_test(){
    let ih = InteractionHandler::new(PUB_KEY);

    let mut app = interaction_app_init!(ih);

    let req = test::TestRequest::post()
        .uri("/api/discord/interactions")
        .header("Content-Type", "application/json")
        .header("X-Signature-Ed25519", "69696969696969696696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696696969")
        .set_payload("This is some malformed text { the system : can't really handle }")
        .to_request();

    let res: types::MessageError = test::read_response_json(&mut app, req).await;

    assert_eq!(res.message, "Bad signature data");

}

#[actix_rt::test]
// Request with missing a signature that is too short (< 512 bits)
// Expected result: Return 400 without panicking
async fn interactions_bad_signature_length_short_test(){
    let ih = InteractionHandler::new(PUB_KEY);

    let mut app = interaction_app_init!(ih);

    let req = test::TestRequest::post()
        .uri("/api/discord/interactions")
        .header("Content-Type", "application/json")
        .header("X-Signature-Ed25519", "69696969696969696696969696969696969696969696969696969696969696969")
        .set_payload("This is some malformed text { the system : can't really handle }")
        .to_request();

    let res: types::MessageError = test::read_response_json(&mut app, req).await;

    assert_eq!(res.message, "Bad signature data");

}

#[actix_rt::test]
// Request with missing a signature that is too long (> 512 bits)
// Expected result: Return 400 without panicking
async fn interactions_bad_signature_length_long_test(){
    let ih = InteractionHandler::new(PUB_KEY);

    let mut app = interaction_app_init!(ih);

    let req = test::TestRequest::post()
        .uri("/api/discord/interactions")
        .header("Content-Type", "application/json")
        .header("X-Signature-Ed25519", "6969696969696969669696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969669696969696969696969696696969696969696969696969696969696969696969")
        .set_payload("This is some malformed text { the system : can't really handle }")
        .to_request();

    let res: types::MessageError = test::read_response_json(&mut app, req).await;

    assert_eq!(res.message, "Bad signature data");

}

#[actix_rt::test]
// Request with missing X-Signature Header
// Expected result: Return 200 with payload
async fn interactions_ping_test(){
    let ih = InteractionHandler::new(PUB_KEY);

    let mut app = interaction_app_init!(ih);

    let req = test::TestRequest::post()
        .uri("/api/discord/interactions")
        .header("Content-Type", "application/json")
        .header("X-Signature-Ed25519", "c41278a0cf22bf8f3061756063cd7ef548a3df23d0ffc5496209aa0ad4d9593343801bf11e099f41bca1afcac2c70734eebafede3dec7aac1caa5d8fade5af0c")
        .header("X-Signature-Timestamp", "1616343571")
        .set_payload("{\"type\" : 1}")
        .to_request();

    let res: types::InteractionResponse = test::read_response_json(&mut app, req).await;

    assert_eq!(res.r#type, types::InteractionResponseType::PONG);

}