use mongodb::bson::{oid::ObjectId, Array};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub category: String,
    pub text: String,
    pub response: String,
    pub suggestions: Array,
}