use std::{collections::HashMap, str::FromStr, time::SystemTime};

use crate::axum::{
    body::Body,
    extract::Request,
    http::{header::AUTHORIZATION, Response, StatusCode},
    middleware::Next,
};
use rest_api::Signature;
use tracing::instrument;

fn now() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Authorization enum
///
/// # Variants
///
/// * `UserSig(Signature)` - User signature
/// * `Bearer(String)` - Bearer token
/// * `Basic` - Basic authentication
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Authorization {
    UserSig(Signature),
    Bearer {
        token: String,
        claims: HashMap<String, String>,
    },
    Basic {
        username: String,
        password: String,
    },
}

/// Authorization middleware
///
/// # Arguments
///
/// * `req` - The request
/// * `next` - The next middleware
///
/// # Returns
///
/// * `Response<Body>` - The response
/// * `StatusCode` - The status code of the response
#[instrument]
pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    tracing::debug!("Authorization middleware");
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_value) = auth_header.to_str() {
            let mut auth_value = auth_value.split_whitespace();
            let (scheme, value) = (auth_value.next(), auth_value.next());
            match scheme.unwrap_or_default().to_lowercase().as_str() {
                "usersig" => {
                    tracing::debug!("User signature");
                    let sig = verify_usersig(value)?;
                    req.extensions_mut()
                        .insert(Some(Authorization::UserSig(sig)));
                    return Ok(next.run(req).await);
                }
                _ => {
                    tracing::debug!("Unknown scheme: {}", scheme.unwrap_or_default());
                }
            }
        }
    }

    tracing::debug!("No Authorization header");
    req.extensions_mut().insert(None::<Authorization>);

    return Ok(next.run(req).await);
}

/// Verify the user signature
/// You must set the `DOMAIN` environment.
/// The signature is valid for 1 hour.
/// The signature is in the format of `timestamp:signature` for authorization header.
/// The signature is verified using the message `domain-timestamp` and the public key of the user.
///
/// # Arguments
///
/// * `value` - The value of the Authorization header
///
/// # Returns
///
/// * `Signature` - The signature of the user
/// * `StatusCode` - The status code of the response
pub fn verify_usersig(value: Option<&str>) -> Result<Signature, StatusCode> {
    if let Some((timestamp, signature)) = value.unwrap_or_default().split_once(":") {
        let parsed_timestamp: i64 = timestamp.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;
        if now() - parsed_timestamp >= 3600 {
            tracing::error!("Expired timestamp: {}", timestamp);
            return Err(StatusCode::UNAUTHORIZED);
        }

        let msg = format!(
            "{}-{}",
            option_env!("DOMAIN").expect("You must set DOMAIN environment"),
            timestamp
        );
        let sig = rest_api::Signature::from_str(signature).map_err(|e| {
            tracing::error!("Failed to parse signature: {}", e);
            StatusCode::UNAUTHORIZED
        })?;
        tracing::debug!("SignMessage: {}", msg);
        let address = sig.verify(&msg).map_err(|e| {
            tracing::error!("Failed to verify signature: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

        if address.is_empty() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        return Ok(sig);
    }

    Err(StatusCode::UNAUTHORIZED)
}
