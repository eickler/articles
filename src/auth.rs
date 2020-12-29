use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::{Deserialize, Serialize};

const METHOD: &str = "Bearer ";
const KEY: &[u8] = b"MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0NGR6UhmJuR1uwQCJMhwa7xxF2F+CrNTD+gupMEYhVkA8R6qae9po76QY3Z7gefdbzzE+/N7erYdb8tQoY0l8+v7wigIX83nZwN/EaST5oa/CYcWp9IcPV0tJUNio6qCiPcnbYmG0+2gNXIauEpVxslJxdiok0miePsyRZew+jGIhXPDHVbLmWzMkULsVevZ6uwIymHA2P/iCD+Sdotsz7Kc6nNIaLfO86nCIfKJuZmFsCHcbfSAJRsIxYmHSjei+drE2mrxFBaDh2+tIVoeBf9ouWGEoTSStzeKMyuX/TbPz1MQdTdRUT3/RsWq4S/bx9GyhjhM320F+TNK8XsuDQIDAQAB";

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub exp: i64,
    pub id: i32,
    pub username: String,
}

// TODO: There is no error information passed back to the client. This was 
// quite difficult to do because it requires a mixture or Option and Result, 
// then Error in jsonwebtoken is private and there's lifetimes.

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        match request.headers().get_one("authorization").and_then(extract_token).and_then(decode_token) {
            Some(token_data) => Outcome::Success(token_data.claims),
            None => Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_token(header: &str) -> Option<&str> {
    if header.starts_with(METHOD) {
        Some(&header[METHOD.len()..])
    } else {
        None
    }
}

fn decode_token(token: &str) -> Option<TokenData<Auth>> {
    let validation = Validation::default();
    match decode::<Auth>(&token, &DecodingKey::from_secret(KEY), &validation) {
        Ok(token_data) => Some(token_data),
        Err(_) => None
    }
}
