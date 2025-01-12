use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::bson::{doc, oid::ObjectId};
use wither::Model as WitherModel;

use crate::utils::models::ModelExt;

impl ModelExt for Question {}

#[derive(Debug, Clone, Serialize, Deserialize, WitherModel, Validate)]
pub struct Question {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub room_id: ObjectId,
    pub answered: bool,
    pub reaction_count: u16,
    pub value: String,
}

impl Question {
    pub fn new(room_id: ObjectId, value: String) -> Self {
        Self {
            id: None,
            room_id,
            answered: false,
            reaction_count: 0,
            value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicQuestion {
    #[serde(alias = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub answered: bool,
    pub reaction_count: u16,
    pub value: String,
}

impl From<Question> for PublicQuestion {
    fn from(question: Question) -> Self {
        Self {
            id: question.id.unwrap(),
            answered: question.answered,
            reaction_count: question.reaction_count,
            value: question.value,
        }
    }
}
