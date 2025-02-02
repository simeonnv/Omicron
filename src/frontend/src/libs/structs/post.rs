use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Debug)]
pub struct PostStruct {
    pub body: String,
    pub created_at: String,
    pub embed_id: Option<i64>,
    pub header: String,
    pub post_id: i64,
    pub poster_id: i64,
    pub subicron_id: i64,
    pub poster_username: String,
    pub upvotes: i64,
    pub is_upvoted: bool
}