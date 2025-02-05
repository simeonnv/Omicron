use serde::{Deserialize, Serialize};
use yew::Properties;



#[derive(Deserialize, Serialize, Debug, Properties, PartialEq, Clone, Default)]
pub struct FileStruct {
    pub file_id: i64,
    pub file_blob: Vec<u8>,
    pub size: i64,
    pub file_type: String,
    pub account_id: i64,
    pub created_at: String
}