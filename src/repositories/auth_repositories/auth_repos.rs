use mongodb::{
    Database,
    Collection,
    bson::extjson::de::Error
};

use crate::{
    models::user::User,
    configurations::database::database_config,
    schemas::requests::auth_requests::LoginRequest
};

use super::{
    login_repository::login_repo,
    register_repository::register_repo,
    refresh_token_repository::refresh_token_repo
};

pub struct AuthRepositories {
    connection: Database,
}
 
impl AuthRepositories {
    pub async fn init() -> Self {
        let connection = database_config().await;
        AuthRepositories { connection }
    }

    /**
     *  Register Repository
     */
    pub async fn register_repository(&self, user_data: User) -> Result<bool, Error> {
        // Collection
        let user_collection: Collection<User> = self.connection
            .collection("User");

        return register_repo(user_data, user_collection).await;
    }

    /**
     *  Login Repository
     */
    pub async fn login_repository(&self, user_data: LoginRequest) -> Result<String, Error> {
        // Collection
        let user_collection: Collection<User> = self.connection
            .collection("User");

        return login_repo(user_data, user_collection).await;
    }


    /**
     *  Refresh Token Repository
     */
    pub async fn refresh_token_repository(&self, user_id: &String) -> Result<bool, Error> {
        // Collection
        let user_collection: Collection<User> = self.connection
            .collection("User");

        return refresh_token_repo(user_id, user_collection).await;
    }
}
