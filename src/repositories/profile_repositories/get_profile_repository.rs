use mongodb::{
    bson::{
        doc,
        oid::ObjectId,
        extjson::de::Error, Document
    },
    Collection
};

use crate::models::user::User;

/**
 *  Get Profile Repository
 */

pub async fn get_profile_repo(
    user_id: &String,
    user_collection: Collection<User>
) -> Result<User, Error> {
    // Parse Id to Object Id
    let obj_id: ObjectId = ObjectId::parse_str(user_id)
        .unwrap();

    // Document Object Id
    let doc_obj_id: Document = doc! {"_id": obj_id};

    // Find user
    let find_user = user_collection.find_one(doc_obj_id, None)
        .await
        .ok()
        .expect("fail when find id");

    Ok(User { 
        id: find_user.as_ref().unwrap().id.clone(), 
        nickname: find_user.as_ref().unwrap().nickname.clone(), 
        email: find_user.as_ref().unwrap().email.clone(), 
        password: find_user.as_ref().unwrap().password.clone(), 
        updated_at: find_user.as_ref().unwrap().updated_at.clone(), 
        created_at: find_user.as_ref().unwrap().created_at.clone()
    })
}
