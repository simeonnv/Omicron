use serde::{Deserialize, Serialize};
use yew::Properties;



#[derive(Deserialize, Serialize, Debug, Properties, PartialEq, Clone, Default)]
pub struct CommentStruct {
    pub text: String,
    pub embed_id: Option<i64>,
    pub created_at: String,
    pub comment_id: i64,
    pub commenter_username: String
}