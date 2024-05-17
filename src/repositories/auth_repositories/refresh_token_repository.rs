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
 *  Refresh Token Repository
 */
pub async fn refresh_token_repo(
    user_id: &String,
    user_collection: Collection<User>
) -> Result<bool, Error> {
    // Parse Id to Object Id
    let obj_id: ObjectId = ObjectId::parse_str(user_id)
        .unwrap();

    // Document Object Id
    let doc_obj_id: Document = doc! {"_id": obj_id};

    // Fine Id
    let find_id = user_collection.find_one(doc_obj_id, None)
        .await.ok()
        .expect("fail when find id");

    if find_id.is_none() {
        return Ok(false)
    }

    Ok(true)
}