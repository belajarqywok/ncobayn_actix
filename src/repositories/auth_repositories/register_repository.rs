use mongodb::{
    bson::{
        doc,
        extjson::de::Error
    },
    Collection
};

use crate::models::user::User;

/**
 *  Register Repository
 */

pub async fn register_repo(
    user_data: User,
    user_collection: Collection<User>
) -> Result<bool, Error> {
    // Find User By Email
    if !user_collection.find_one(
        doc!{"email": &user_data.email}, None
    )
        .await.ok()
        .expect("fail when fine email").is_none() {
            
        // Return False
        return Ok(false);
    }

    // User Data
    let data = User {
        id: None,
        nickname: user_data.nickname,
        email: user_data.email,
        password: user_data.password,
        created_at: user_data.created_at,
        updated_at: user_data.updated_at
    };

    // Insert User data
    user_collection.insert_one(data, None)
        .await
        .ok()
        .expect("Error creating user");

    Ok(true)
}
