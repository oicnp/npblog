

pub struct Article {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub thumb: String,
    pub type_: String,
    pub category: i32,
    pub excerpt: String,
    pub content: String,
    pub status: i8,
    pub push_front: i8,
    pub created_at: i32,
    pub updated_at: i32,
}