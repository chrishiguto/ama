use std::{
    pin::pin,
    time::{Duration, Instant},
};

use actix_ws::{AggregatedMessage, ProtocolError};
use futures_util::{
    future::{select, Either},
    StreamExt as _,
};
use tokio::{sync::mpsc, time::interval};

use crate::server::RoomServerHandle;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

enum MessageSource {
    Client(AggregatedMessage),
    Participant(String),
    Heartbeat,
    ClientError(ProtocolError),
    StreamEnd,
}

pub async fn room_subscribe_handle(
    room_server: RoomServerHandle,
    mut session: actix_ws::Session,
    msg_stream: actix_ws::MessageStream,
    room_id: String,
) {
    log::info!("connected");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    // unwrap: room server is not dropped before the HTTP server
    let session_id = room_server.connect(conn_tx, room_id.to_owned()).await;

    let msg_stream = msg_stream
        .max_frame_size(128 * 1024)
        .aggregate_continuations()
        .max_continuation_size(2 * 1024 * 1024);

    let mut msg_stream = pin!(msg_stream);

    let close_reason = loop {
        // most of the futures we process need to be stack-pinned to work with select()
        let tick = pin!(interval.tick());
        let msg_rx = pin!(conn_rx.recv());

        let messages = pin!(select(msg_stream.next(), msg_rx));

        let message_source = match select(messages, tick).await {
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => MessageSource::Client(msg),
            Either::Left((Either::Left((Some(Err(err)), _)), _)) => MessageSource::ClientError(err),
            Either::Left((Either::Left((None, _)), _)) => MessageSource::StreamEnd,
            Either::Left((Either::Right((Some(msg), _)), _)) => MessageSource::Participant(msg),
            Either::Left((Either::Right((None, _)), _)) => unreachable!(),
            Either::Right((_inst, _)) => MessageSource::Heartbeat,
        };

        match message_source {
            MessageSource::Client(msg) => {
                match msg {
                    AggregatedMessage::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        // unwrap:
                        session.pong(&bytes).await.unwrap();
                    }
                    AggregatedMessage::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }
                    AggregatedMessage::Text(_text) => {
                        // message sent by the client
                    }
                    AggregatedMessage::Binary(_bin) => {
                        log::warn!("unexpected binary message");
                    }
                    AggregatedMessage::Close(reason) => break reason,
                }
            }
            MessageSource::Participant(text) => {
                session.text(text).await.unwrap();
            }
            MessageSource::Heartbeat => {
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    break None;
                }
                let _ = session.ping(b"").await;
            }
            MessageSource::ClientError(err) => {
                log::error!("{}", err);
                break None;
            }
            MessageSource::StreamEnd => break None,
        }
    };

    room_server.disconnect(session_id);

    let _ = session.close(close_reason).await;
}
