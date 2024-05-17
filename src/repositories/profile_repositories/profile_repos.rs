use mongodb::{
    Database,
    Collection,
    bson::extjson::de::Error
};

use crate::{
    models::user::User,
    configurations::database::database_config
};

use super::get_profile_repository::get_profile_repo;

pub struct ProfileRepositories {
    connection: Database,
}

impl ProfileRepositories {
    pub async fn init() -> Self {
        let connection = database_config().await;
        ProfileRepositories { connection }
    }

    // Get Profile Repository
    pub async fn get_profile_repository(&self, user_id: &String) -> Result<User, Error> {
        // Collection
        let user_collection: Collection<User> = self.connection
            .collection("User");

        return get_profile_repo(user_id, user_collection).await;
    }
}