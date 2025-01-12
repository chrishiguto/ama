use actix_web::{
    get, post,
    web::{self, Path},
    Error as ActixWebError, HttpRequest, HttpResponse,
};
use bson::doc;
use mime::APPLICATION_JSON;
use serde::Deserialize;
use tokio::task::spawn_local;
use wither::mongodb::options::FindOptions;

use crate::{
    errors::Error,
    handler::room_subscribe_handle,
    models::{
        question::{PublicQuestion, Question},
        room::{PublicRoom, Room},
    },
    server::RoomServerHandle,
    utils::{models::ModelExt, to_object_id::to_object_id},
};

pub fn create_routes(config: &mut web::ServiceConfig) {
    config
        .service(create_room)
        .service(get_room_by_id)
        .service(query_questions)
        .service(room_subscribe);
}

#[get("/room/{id}")]
pub async fn get_room_by_id(path: Path<String>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let room_id =
        to_object_id(id).map_err(|_| Error::ParseObjectID("Failed to parse object id".into()))?;
    let found_room = Room::find_by_id(&room_id).await?;

    match found_room {
        Some(room) => Ok(HttpResponse::Ok().json(PublicRoom::from(room))),
        None => Err(Error::NotFound("Room not found".into())),
    }
}

#[get("/room/{id}/questions")]
pub async fn query_questions(path: Path<String>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let room_id =
        to_object_id(id).map_err(|_| Error::ParseObjectID("Failed to parse object id".into()))?;

    if Room::find_by_id(&room_id).await?.is_none() {
        return Err(Error::NotFound("Room not found".into()));
    }

    let options = FindOptions::builder()
        .sort(doc! { "created_at": -1_i32  })
        .build();

    let (questions, _count) =
        Question::find_and_count(doc! { "room_id": room_id }, options).await?;

    let questions = questions
        .into_iter()
        .map(Into::into)
        .collect::<Vec<PublicQuestion>>();

    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(questions))
}

#[post("/room")]
async fn create_room(json: web::Json<CreateRoom>) -> Result<HttpResponse, Error> {
    let room = Room::new(json.name.clone());
    let room = Room::create(room).await?;
    let public_room = PublicRoom::from(room);

    Ok(HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(public_room))
}

#[get("/room/subscribe/{room_id}")]
async fn room_subscribe(
    req: HttpRequest,
    stream: web::Payload,
    path: Path<String>,
    room_server: web::Data<RoomServerHandle>,
) -> Result<HttpResponse, ActixWebError> {
    let room_id = path.into_inner();
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    spawn_local(room_subscribe_handle(
        (**room_server).clone(),
        session,
        msg_stream,
        room_id,
    ));

    Ok(res)
}

#[derive(Debug, Clone, Deserialize)]
struct CreateRoom {
    name: String,
}
