use std::env;
extern crate dotenv;
use dotenv::dotenv;

use std::time::{
    Duration,
    SystemTime
};

use jsonwebtoken::{
    decode,
    encode,
    Header,
    Validation,
    DecodingKey,
    EncodingKey
};

use serde::{
    Deserialize,
    Serialize
};

// Claims Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}


/**
 * 
 *  Initialize Secret Keys
 * 
 */

// Secret Key Struct
struct SecretKeyStruct {
    access_key: String,
    refresh_key: String,
}

// Get Secret Keys
fn secret_key() -> Result<SecretKeyStruct, Box<dyn std::error::Error>> {
    dotenv().ok();

    // Access Key
    let secret_access_key = match env::var("SECRET_ACCESS_KEY") {
        Ok(v) => v,
        Err(_) => return Err(
            Box::new(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error loading env variable"
                )
            )
        ),
    };

    // Refresh Key
    let secret_refresh_key = match env::var("SECRET_REFRESH_KEY") {
        Ok(v) => v,
        Err(_) => return Err(
            Box::new(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error loading env variable"
                )
            )
        ),
    };

    Ok(
        SecretKeyStruct {
            access_key:  secret_access_key,
            refresh_key: secret_refresh_key,
        }
    )
}



/**
 * 
 *  Generate Tokens:
 *      - generate access token
 *      - generate refresh token
 * 
 */

// Generate Access Token
pub async fn generate_access_token(id: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Token expires in 1 hour
    let expiration = SystemTime::now() + Duration::from_secs(3600); 

    // Claims
    let claims = Claims {
        sub: id.to_owned(),
        exp: expiration.duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs() as usize,
    };

    // Secret Key
    let secret_key = match secret_key() {
        Ok(secret) => secret.access_key,
        Err(e) => return Err(e.into()),
    };

    // Token
    let token = encode(
        &Header::default(), &claims,
        &EncodingKey::from_secret(&secret_key.as_bytes())
    )?;

    Ok(token)
}

// Generate Refresh Token
pub async fn generate_refresh_token(id: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Token expires in 1 week
    let expiration = SystemTime::now() + (
        Duration::from_secs(3600) * 24 * 7
    ); 

    // Claims
    let claims = Claims {
        sub: id.to_owned(),
        exp: expiration.duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs() as usize,
    };

    // Secret Key
    let secret_key = match secret_key() {
        Ok(secret) => secret.refresh_key,
        Err(e) => return Err(e.into()),
    };

    // Token
    let token = encode(
        &Header::default(), &claims,
        &EncodingKey::from_secret(&secret_key.as_bytes())
    )?;

    Ok(token)
}


/**
 * 
 *  Decode Access and Refresh Token
 * 
 */

// Decode Access Token
pub async fn decode_access_token(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    // Secret Key
    let secret_key = match secret_key() {
        Ok(secret) => secret.access_key,
        Err(e) => return Err(e.into()),
    };

    // Decoding Key
    let decoding_key = DecodingKey::from_secret(
        &secret_key.as_bytes()
    );

    // Validation
    let validation = Validation::default();

    // Decoding Token
    let decoding_token = decode::<Claims>(
        token, &decoding_key, &validation
    )?;

    Ok(decoding_token.claims)
}

// Decode Refresh Token
pub async fn decode_refresh_token(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    // Secret Key
    let secret_key = match secret_key() {
        Ok(secret) => secret.refresh_key,
        Err(e) => return Err(e.into()),
    };

    // Decoding Key
    let decoding_key = DecodingKey::from_secret(
        &secret_key.as_bytes()
    );

    // Validation
    let validation = Validation::default();

    // Decoding Token
    let decoding_token = decode::<Claims>(
        token, &decoding_key, &validation
    )?;

    Ok(decoding_token.claims)
}