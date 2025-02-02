use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubicronStruct {
    pub created_at: String,
    pub image_id: Option<i64>,
    pub name: String,
    pub subicron_id: i64
}  