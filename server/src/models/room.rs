use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::bson::{doc, oid::ObjectId};
use wither::Model as WitherModel;

use crate::utils::models::ModelExt;

impl ModelExt for Room {}

#[derive(Debug, Serialize, Deserialize, WitherModel, Validate)]
pub struct Room {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(length(min = 1, message = "Room name cannot be empty"))]
    pub name: String,
    pub questions_count: i8,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            questions_count: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicRoom {
    #[serde(alias = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub questions_count: i8,
}

impl From<Room> for PublicRoom {
    fn from(room: Room) -> Self {
        Self {
            id: room.id.unwrap(),
            name: room.name,
            questions_count: room.questions_count,
        }
    }
}
