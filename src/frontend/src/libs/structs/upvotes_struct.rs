use serde::{Deserialize, Serialize};
use yew::Properties;



#[derive(Deserialize, Serialize, Debug, Properties, PartialEq, Clone, Default)]
pub struct UpvotesStruct {
    pub is_upvoted: bool,
    pub upvotes: i64
}