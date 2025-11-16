use crate::common_types::{DatabaseError, PublicId, ShopDB};
use crate::models::user_model::{UserId, UserModel};
use crate::repository_traits::user_repository_trait::{CreatedUser, NewUser, UserRepositoryTrait};
use async_trait::async_trait;
use sqlx::Row;
use crate::config::db_config::start_transaction;
use crate::models::user_credential_model::UserCredentialModel;

pub struct UserRepository {
    pub(crate) shop_db: ShopDB,
}

impl UserRepository {
    pub fn new(shop_db: ShopDB) -> Self {
        Self { shop_db }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create_user(&self, user: NewUser) -> anyhow::Result<CreatedUser, DatabaseError> {
        // start the transaction.
        let mut tx = start_transaction(&self.shop_db).await?;

        let email = user.email;

        // create the user.
        let created_user = sqlx::query(
            "insert  into users (first_name, last_name, email, phone) values ($1, $2, $3, $4) returning id, public_id",
        ).bind(user.first_name)
            .bind(user.last_name)
            .bind(email.clone())
            .bind(user.phone)
            .fetch_one(tx.as_mut())
            .await?;
        let user_id: UserId = created_user.get::<i64, _>("id") as UserId;
        let public_id = PublicId::from(created_user.get::<String, _>("public_id"));

        // store the user credentials.
        sqlx::query("insert  into user_credentials (user_id, hashed_password) values ($1, $2)")
            .bind(user_id)
            .bind(user.hashed_password)
            .execute(tx.as_mut())
            .await?;
        tx.commit().await?;

        Ok(CreatedUser { email, public_id })
    }

    async fn get_user_by_id(&self, user_id: &UserId) -> anyhow::Result<Option<UserModel>, DatabaseError> {
        unimplemented!()
    }

    async fn get_user_by_public_id(&self, public_id: &PublicId) -> anyhow::Result<Option<UserModel>, DatabaseError> {
        let user: Option<UserModel> = sqlx::query_as("select * from users where public_id = $1")
            .bind(public_id)
            .fetch_optional(&self.shop_db).await?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: &String) -> anyhow::Result<Option<UserModel>, DatabaseError> {
        let user: Option<UserModel> = sqlx::query_as("select * from users where email = $1")
            .bind(email)
            .fetch_optional(&self.shop_db).await?;
        Ok(user)
    }

    async fn get_user_credentials_by_id(&self, user_id: &UserId) -> anyhow::Result<Option<UserCredentialModel>, DatabaseError> {
        let user_credential: Option<UserCredentialModel> = sqlx::query_as("select * from user_credentials where user_id =$1")
            .bind(user_id)
            .fetch_optional(&self.shop_db).await?;
        Ok(user_credential)
    }
}
