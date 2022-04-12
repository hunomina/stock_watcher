use futures::stream::{StreamExt, TryStreamExt};

use mongodb::{
    bson::doc, error::Error, options::FindOneOptions, results::InsertOneResult, Collection,
    Database,
};
use serde::{Deserialize, Serialize};
use serenity::model::id::ChannelId;

type Subscription = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: u64,
    pub subscriptions: Vec<Subscription>,
}

impl Subscriber {
    pub fn new(channel_id: ChannelId) -> Self {
        Subscriber {
            id: channel_id.0,
            subscriptions: vec![],
        }
    }
}

pub struct SubscriberRepository {
    pub database_connection: Database,
}

impl SubscriberRepository {
    pub async fn all(&self) -> Result<Vec<Subscriber>, Error> {
        self.get_collection()
            .find(None, None)
            .await?
            .try_collect()
            .await
    }

    pub async fn add(&self, subscriber: Subscriber) -> Result<InsertOneResult, Error> {
        self.get_collection().insert_one(subscriber, None).await
    }

    pub fn remove(&self, subscriber: Subscriber) {
        unimplemented!();
    }

    pub async fn get(&self, id: u64) -> Result<Option<Subscriber>, Error> {
        self.get_collection()
            .find_one(doc! { "id": id as i64 }, None)
            .await
    }

    fn get_collection(&self) -> Collection<Subscriber> {
        self.database_connection.collection("subscriber")
    }
}
