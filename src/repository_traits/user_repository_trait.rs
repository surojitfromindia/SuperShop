
struct UserRegister {
    email: String,
    password: String,
}

trait  UserRepositoryTrait {
    async fn register_user();
    async fn get_user_by_id();
    async fn get_user_by_public_id();
    async fn get_login_details_by_email();
}