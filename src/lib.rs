/*
config:
---------------
hold database config and app state.
an app state must be created first to use the store backend. this app state also load
some pre-defined data from env, failing to do show will crash the app.

models:
---------------
contains database models.


repository_traits:
----------------
hold method exposed by database on those models and there return type can be found here.

repository:
----------------
hold actual impl of the repository_traits.

services:
---------------
core business logic.

schemas
---------------
input/output struct into the service. this struct will be exposed to
external handlers to implement to and form.
 */

mod common_types;

mod config;
mod models;
mod repositories;
mod repository_traits;
mod schemas;
mod services;

mod utils;

pub use config::app_state::AppState;

pub use config::db_config::DbConfig;

pub use services::user_service::UserService;

pub mod types {
    pub use crate::common_types::{PrimaryId, PublicId};
    pub use crate::services::user_service::{RegisterUserInput, RegisterUserOutput};
    pub use crate::services::auth_service::{UserLoginError, UserLoginInput, UserLoginOutput};
    pub use crate::utils::password_util::PlainPassword;
}

pub mod errors {
    pub use crate::services::auth_service::AuthService;
    pub use crate::services::user_service::RegisterUserError;
}
