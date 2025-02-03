use serde::{Deserialize, Serialize};
use yew::Properties;



#[derive(Deserialize, Serialize, Debug, Properties, PartialEq, Clone)]
pub struct UpvotesStruct {
    is_upvoted: bool,
    upvotes: i64
}