use crate::AppState;
use crate::repository_traits::user_repository_trait::{CreatedUser, NewUser, UserRepositoryTrait};

pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
}

pub struct UserService {
    pub app_state: AppState,
}

// todo: some controller errors will be thrown from particular services;
impl UserService {
    pub async fn register_user(&self, user: RegisterUser) -> CreatedUser {
        // todo: hash the password
        let hash_password = user.password;
        let new_user = NewUser {
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            hash_password,
        };
        self.app_state
            .repositories
            .user_repository
            .create_user(new_user)
            .await
            .unwrap()
    }
}
