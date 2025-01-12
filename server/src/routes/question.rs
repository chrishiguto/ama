use actix_web::{
    delete, get, patch, post,
    web::{self, Path},
    HttpResponse,
};
use bson::{bson, doc};
use mime::APPLICATION_JSON;
use serde::{Deserialize, Serialize};

use crate::{
    errors::Error,
    models::question::{PublicQuestion, Question},
    server::RoomServerHandle,
    utils::{message_data::MessageData, models::ModelExt, to_object_id::to_object_id},
};

pub fn create_routes(config: &mut web::ServiceConfig) {
    config
        .service(create_question)
        .service(get_question_by_id)
        .service(answer_question)
        .service(react_question);
}

#[get("/question/{id}")]
pub async fn get_question_by_id(path: Path<String>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let question_id =
        to_object_id(id).map_err(|_| Error::ParseObjectID("Failed to parse object id".into()))?;
    let found_question = Question::find_by_id(&question_id).await?;

    match found_question {
        Some(question) => Ok(HttpResponse::Ok().json(PublicQuestion::from(question))),
        None => Err(Error::NotFound("Question not found".into())),
    }
}

#[patch("/question/{id}/answer")]
async fn answer_question(
    path: Path<String>,
    room_server: web::Data<RoomServerHandle>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let question_id =
        to_object_id(id).map_err(|_| Error::ParseObjectID("Failed to parse object id".into()))?;

    let updated_question = Question::find_one_and_update(
        doc! { "_id": question_id },
        doc! { "$set": bson!({
            "answered": true
        }) },
    )
    .await?;

    match updated_question {
        Some(question) => {
            let public_question = PublicQuestion::from(question.clone());
            let msg_data = MessageData::update(&public_question);
            room_server
                .send_message(question.room_id.to_string(), msg_data)
                .await;

            Ok(HttpResponse::Ok().json(PublicQuestion::from(question)))
        }
        None => Err(Error::NotFound("Question not found".into())),
    }
}

#[delete("/question/{id}/answer")]
async fn delete_answer_question(
    path: Path<String>,
    room_server: web::Data<RoomServerHandle>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let question_id =
        to_object_id(id).map_err(|_| Error::ParseObjectID("Failed to parse object id".into()))?;

    let updated_question = Question::find_one_and_update(
        doc! { "_id": question_id },
        doc! { "$set": bson!({
            "answered": false
        }) },
    )
    .await?;

    match updated_question {
        Some(question) => {
            let public_question = PublicQuestion::from(question.clone());
            let msg_data = MessageData::update(&public_question);
            room_server
                .send_message(question.room_id.to_string(), msg_data)
                .await;

            Ok(HttpResponse::Ok().json(PublicQuestion::from(question)))
        }
        None => Err(Error::NotFound("Question not found".into())),
    }
}

#[patch("/question/{id}/react")]
async fn react_question(
    path: Path<String>,
    room_server: web::Data<RoomServerHandle>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let question_id =
        to_object_id(id).map_err(|_| Error::ParseObjectID("Failed to parse object id".into()))?;

    let updated_question = Question::find_one_and_update(
        doc! { "_id": question_id },
        doc! { "$inc": bson!({
            "reaction_count": 1
        }) },
    )
    .await?;

    match updated_question {
        Some(question) => {
            let public_question = PublicQuestion::from(question.clone());
            let msg_data = MessageData::update(&public_question);
            room_server
                .send_message(question.room_id.to_string(), msg_data)
                .await;

            Ok(HttpResponse::Ok().json(PublicQuestion::from(question)))
        }
        None => Err(Error::NotFound("Question not found".into())),
    }
}

#[delete("/question/{id}/react")]
async fn remove_react_question(
    path: Path<String>,
    room_server: web::Data<RoomServerHandle>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let question_id =
        to_object_id(id).map_err(|_| Error::ParseObjectID("Failed to parse object id".into()))?;

    let updated_question = Question::find_one_and_update(
        doc! { "_id": question_id },
        doc! { "$dec": bson!({
            "reaction_count": 1
        }) },
    )
    .await?;

    match updated_question {
        Some(question) => {
            let public_question = PublicQuestion::from(question.clone());
            let msg_data = MessageData::update(&public_question);
            room_server
                .send_message(question.room_id.to_string(), msg_data)
                .await;

            Ok(HttpResponse::Ok().json(PublicQuestion::from(question)))
        }
        None => Err(Error::NotFound("Question not found".into())),
    }
}

#[post("/question")]
async fn create_question(
    json: web::Json<CreateQuestion>,
    room_server: web::Data<RoomServerHandle>,
) -> Result<HttpResponse, Error> {
    let body = json.clone();

    let room_id = to_object_id(body.room_id.clone())
        .map_err(|_| Error::ParseObjectID("Failed to parse object id".into()))?;

    let question = Question::new(room_id, body.value);
    let question = Question::create(question).await;

    let public_question =
        question.map_err(|_| Error::InternalServerError("Failed to map question".into()))?;
    let public_question = PublicQuestion::from(public_question);

    let msg_data = MessageData::create(&public_question);
    room_server.send_message(body.room_id, msg_data).await;

    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(public_question))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateQuestion {
    answered: Option<bool>,
    reaction_count: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuestion {
    room_id: String,
    value: String,
}
