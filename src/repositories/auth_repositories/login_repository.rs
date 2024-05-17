use mongodb::{
    bson::{
        doc, 
        oid::ObjectId, 
        extjson::de::Error
    },
    Collection
};

use crate::{
    models::user::User,
    schemas::requests::auth_requests::LoginRequest
};


/**
 *  Login Repository
 */
pub async fn login_repo(
    user_data: LoginRequest,
    user_collection: Collection<User>
) -> Result<String, Error> {

    // Credential
    let credential = doc! {
        "email": &user_data.email,
        "password": &user_data.password
    };

    // Find User By Credential
    let find_user = user_collection.find_one(credential, None)
        .await.ok()
        .expect("fail when find credential");

    
    if find_user.is_none() {
        // Return False
        return Ok("0".to_string());
    }

    Ok(ObjectId::to_string(
            &find_user.unwrap().id.unwrap()
        )
    )
}
