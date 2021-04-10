use crate::security::*;
use crate::types::*;
use actix_web::http::{StatusCode};
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result,};
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use ed25519_dalek::{PublicKey};