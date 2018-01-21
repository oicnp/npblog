

pub struct User {
    pub id: i32,
    pub email: String,
    pub real_name: String,
    pub password_hash: String,
    pub password_reset_token: String,
    pub is_admin: i8,
    pub created_at: i32,
    pub updated_at: i32,
}