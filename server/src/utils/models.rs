use async_trait::async_trait;
use futures::stream::TryStreamExt;
use validator::Validate;
use wither::bson::{doc, oid::ObjectId, Document};
use wither::mongodb::options::FindOneAndUpdateOptions;
use wither::mongodb::options::FindOptions;
use wither::mongodb::options::ReturnDocument;
use wither::Model as WitherModel;

use crate::{database, errors::Error};

#[async_trait]
pub trait ModelExt
where
    Self: WitherModel + Validate + Send,
{
    async fn create(mut model: Self) -> Result<Self, Error> {
        let connection = database::connection().await;
        model.validate().map_err(|error| {
            let error_messages: Vec<String> = error
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().filter_map(move |e| {
                        e.message
                            .clone()
                            .map(|message| format!("{}: {}", field, message))
                    })
                })
                .collect();

            Error::bad_request(format!("Validation errors: {:?}", error_messages))
        })?;
        model.save(connection, None).await.map_err(|_error| {
            Error::InternalServerError("Error while creating the resource".into())
        })?;

        Ok(model)
    }

    async fn find_and_count<O>(query: Document, options: O) -> Result<(Vec<Self>, u64), Error>
    where
        O: Into<Option<FindOptions>> + Send,
    {
        let connection = database::connection().await;

        let count = Self::collection(connection)
            .count_documents(query.clone(), None)
            .await
            .map_err(Error::Mongo)?;

        let items = <Self as WitherModel>::find(connection, query, options.into())
            .await
            .map_err(Error::Wither)?
            .try_collect::<Vec<Self>>()
            .await
            .map_err(Error::Wither)?;

        Ok((items, count))
    }

    async fn find_by_id(id: &ObjectId) -> Result<Option<Self>, Error> {
        let connection = database::connection().await;
        <Self as WitherModel>::find_one(connection, doc! { "_id": id }, None)
            .await
            .map_err(|_| Error::NotFound("Error while fetching the room".into()))
    }

    async fn find_one_and_update(query: Document, update: Document) -> Result<Option<Self>, Error> {
        let connection = database::connection().await;
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        <Self as WitherModel>::find_one_and_update(connection, query, update, options)
            .await
            .map_err(Error::Wither)
    }
}
