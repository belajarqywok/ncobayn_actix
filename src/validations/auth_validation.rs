use crate::configurations::auth::{
    decode_access_token,
    decode_refresh_token
};

use actix_web::http::header::HeaderValue;

pub struct TokenValidationStruct {
    pub response: bool,
    pub user_id: String,
}

/**
 *  Access Token Validation
 */
pub async fn access_token_validation(
    access_token_header: Option<&HeaderValue>
) -> TokenValidationStruct {
    // Match Access Token
    match access_token_header {
        Some(access_token_value) => {
            // Parse Access Token
            let access_token = access_token_value
                .to_str().unwrap()
                .trim_start_matches("Bearer ");

            match decode_access_token(access_token).await {
                Ok(claims) => return TokenValidationStruct {
                    response: true,
                    user_id: format!("{}", claims.sub)
                },

                Err(_) => return TokenValidationStruct {
                    response: false,
                    user_id: "".to_string()
                }
            }
        }

        None => return TokenValidationStruct {
            response: false,
            user_id: "".to_string()
        }
    }
}


/**
 *  Refresh Token Validation
 */
pub async fn refresh_token_validation(
    refresh_token_header: &str
) -> TokenValidationStruct {
    // Match Refresh Token
    match decode_refresh_token(refresh_token_header).await {
        Ok(claims) => return TokenValidationStruct {
            response: true,
            user_id: format!("{}", claims.sub)
        },

        Err(_) => return TokenValidationStruct {
            response: false,
            user_id: "".to_string()
        }
    }
}
