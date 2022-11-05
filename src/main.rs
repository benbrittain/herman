use anyhow::Error;
use prost::Message;
use proto::transit_realtime::FeedMessage;
use reqwest::{header, Client};
use std::io::Cursor;

const L_FEED: &'static str = "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-l";

// File must not have a trailing newline
const ACCESS_KEY: &'static str = include_str!("../access_key");

fn create_client() -> Result<Client, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("x-api-key", header::HeaderValue::from_static(ACCESS_KEY));
    Client::builder()
        .default_headers(headers)
        .build()
        .map_err(Into::into)
}

async fn get_feed_msg(client: &Client) -> Result<FeedMessage, Error> {
    let body = client.get(L_FEED).send().await?.bytes().await?;
    proto::transit_realtime::FeedMessage::decode(&mut Cursor::new(body)).map_err(Into::into)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = create_client()?;
    let msg = get_feed_msg(&client).await?;
    println!("{:#?}", msg);
    Ok(())
}

pub mod proto {
    pub mod transit_realtime {
        // nyct proto extension included
        include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
    }
}
