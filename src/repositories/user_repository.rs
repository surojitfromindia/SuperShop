use crate::common_types::{PublicId, ShopDB};
use crate::models::user_model::{UserId, UserModel};
use crate::repository_traits::user_repository_trait::{CreatedUser, NewUser, UserRepositoryTrait};
use async_trait::async_trait;
use sqlx::Row;

pub struct UserRepository {
    shop_db: ShopDB,
}

impl UserRepository {
    pub fn new(shop_db: ShopDB) -> Self {
        Self { shop_db }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create_user(&self, user: NewUser) -> anyhow::Result<CreatedUser> {
        // start the transaction.
        let mut tx2 = self.shop_db.begin().await?;

        let email = user.email;

        // create the user.
        let created_user = sqlx::query(
            "insert  into users (first_name, last_name, email, phone) values ($1, $2, $3, $4) returning id, public_id",
        ).bind(user.first_name)
            .bind(user.last_name)
            .bind(email.clone())
            .bind(user.phone)
        .fetch_one(tx2.as_mut())
        .await?;
        let user_id: UserId = created_user.get::<i64, _>("id") as UserId;
        let public_id = PublicId::from(created_user.get::<String, _>("public_id"));

        // store the user credentials.
        sqlx::query("insert  into user_credentials (user_id, hash_password) values ($1, $2)")
            .bind(user_id)
            .bind(user.hash_password)
            .execute(tx2.as_mut())
            .await?;
        tx2.commit().await?;

        Ok(CreatedUser { email, public_id })
    }

    async fn get_user_by_id(&self) -> anyhow::Result<UserModel> {
        unimplemented!()
    }

    async fn get_user_by_public_id(&self) -> anyhow::Result<UserModel> {
        unimplemented!()
    }

    async fn get_user_by_email(&self) -> anyhow::Result<UserModel> {
        unimplemented!()
    }
}
