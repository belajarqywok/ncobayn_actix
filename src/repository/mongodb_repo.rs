use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection, Database
};

use crate::models::user_model::User;
use crate::configurations::database::database_config;

// use sha256::digest;

pub struct MongoRepo {
    db: Database
}

impl MongoRepo {
    pub async fn init() -> Self {
        let db = database_config().await;
        MongoRepo { db }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let col: Collection<User> = self.db.collection("User");
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<Option<User>, Error> {
        let col: Collection<User> = self.db.collection("User");
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(user_detail)
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let col: Collection<User> = self.db.collection("User");
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };

        let updated_doc = col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");

        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let col: Collection<User> = self.db.collection("User");
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let col: Collection<User> = self.db.collection("User");
        let mut cursors = col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }


    // pub async fn register_repo(&self, user_data_request: UserModel) -> Result<InsertOneResult, Error> {
    //     let user_data = UserModel {
    //         id: None,
    //         fullname: user_data_request.fullname,
    //         email:    user_data_request.email,
    //         password: digest(
    //             user_data_request.password
    //         ),
    //     };

    //     let user = self
    //         .col
    //         .insert_one(user_data, None)
    //         .await
    //         .ok()
    //         .expect("Error creating user");

    //     Ok(user)
    // }
}
