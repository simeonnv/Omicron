use serde::{Deserialize, Serialize};
use yew::Properties;



#[derive(Deserialize, Serialize, Debug, Properties, PartialEq, Clone)]
pub struct PostStruct {
    pub body: String,
    pub created_at: String,
    pub embed_id: Option<i64>,
    pub header: String,
    pub post_id: i64,
    pub poster_id: i64,
    pub subicron_id: i64,
    pub poster_username: String
}