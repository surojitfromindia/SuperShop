use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, Transaction};
use std::ops::Deref;
use crate::models::user_model::UserModel;

pub type PrimaryId = i64;

pub type OrganizationId = PrimaryId;
pub type UserId = i64;


// this is public facing id of our entity.
#[derive(Clone, Debug, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PublicId(String);


impl From<String> for PublicId {
    fn from(value: String) -> Self {
        PublicId(value)
    }
}

impl Deref for PublicId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for PublicId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for PublicId {
    fn into(self) -> String {
        self.0
    }
}


pub type CreatedAt = DateTime<Utc>;
pub type UpdatedAt = DateTime<Utc>;


pub type DatabaseError = sqlx::Error;

pub struct Password(String);


pub type ShopDB = PgPool;
pub type DBTransaction<'a> = Transaction<'a, Postgres>;


pub struct PublicIdCounter<'a> {
    pub organization_id: &'a OrganizationId,
    pub organization_public_id: &'a PublicId,
}

