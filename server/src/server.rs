use std::{
    collections::{HashMap, HashSet},
    io,
};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

#[derive(Debug)]
enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<String>,
        res_tx: oneshot::Sender<Uuid>,
        room_id: String,
    },

    Disconnect {
        conn: Uuid,
    },

    Message {
        msg: String,
        room_id: String,
        res_tx: oneshot::Sender<()>,
    },
}

#[derive(Clone)]
pub struct RoomServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

impl RoomServerHandle {
    pub async fn connect(&self, conn_tx: mpsc::UnboundedSender<String>, room_id: String) -> Uuid {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send(Command::Connect {
                conn_tx,
                res_tx,
                room_id,
            })
            .unwrap();

        res_rx.await.unwrap()
    }

    pub fn disconnect(&self, conn: Uuid) {
        // unwrap: room server should not have been dropped
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }

    pub async fn send_message(&self, room_id: String, msg: impl Into<String>) {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send(Command::Message {
                msg: msg.into(),
                room_id,
                res_tx,
            })
            .unwrap();

        res_rx.await.unwrap()
    }
}

#[derive(Debug)]
pub struct RoomServer {
    sessions: HashMap<Uuid, mpsc::UnboundedSender<String>>,
    rooms: HashMap<String, HashSet<Uuid>>,
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

impl RoomServer {
    pub fn new() -> (Self, RoomServerHandle) {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            RoomServer {
                sessions: HashMap::new(),
                rooms: HashMap::new(),
                cmd_rx,
            },
            RoomServerHandle { cmd_tx },
        )
    }

    /// Send message to users in a room.
    ///
    async fn send_system_message(&self, room: &str, msg: impl Into<String>) {
        if let Some(sessions) = self.rooms.get(room) {
            let msg = msg.into();

            for conn_id in sessions {
                if let Some(tx) = self.sessions.get(conn_id) {
                    // errors if client disconnected abruptly and hasn't been timed-out yet
                    let _ = tx.send(msg.clone());
                }
            }
        }
    }

    /// Send message to all other users in current room.
    ///
    async fn send_message(&self, room_id: String, msg: impl Into<String>) {
        if self.rooms.contains_key(&room_id) {
            self.send_system_message(&room_id, msg).await;
            log::info!("message broadcasted to room {room_id}");
        } else {
            log::info!("room not found to broadcast message");
        }
    }

    /// Register new session and assign unique ID to this session
    ///
    async fn connect(&mut self, tx: mpsc::UnboundedSender<String>, room_id: String) -> Uuid {
        self.send_system_message(&room_id, "Someone joined").await;

        let id = Uuid::new_v4();
        self.sessions.insert(id, tx);

        self.rooms.entry(room_id).or_default().insert(id);

        id
    }

    /// Unregister connection from room map and broadcast disconnection message.
    ///
    async fn disconnect(&mut self, session_id: Uuid) {
        println!("Someone disconnected");

        let mut room = String::new();

        // remove sender
        if self.sessions.remove(&session_id).is_some() {
            // remove session from all rooms
            for (room_id, sessions) in &mut self.rooms {
                if sessions.remove(&session_id) {
                    room.clone_from(room_id);
                }
            }
        }

        if !room.is_empty() {
            self.send_system_message(&room, "session {session_id} has disconnected.")
                .await;
        }
    }

    pub async fn run(mut self) -> io::Result<()> {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect {
                    conn_tx,
                    res_tx,
                    room_id,
                } => {
                    let conn_id = self.connect(conn_tx, room_id).await;
                    let _ = res_tx.send(conn_id);
                }
                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }
                Command::Message {
                    room_id,
                    msg,
                    res_tx,
                } => {
                    self.send_message(room_id, msg).await;
                    let _ = res_tx.send(());
                }
            }
        }

        Ok(())
    }
}
