use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}


// #[derive(Debug, Deserialize)]
// pub struct UserModel {
//     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//     pub id: Option<ObjectId>,
//     pub fullname: String,
//     pub email:    String,
//     pub password: String,
// }
