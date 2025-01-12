use serde::Serialize;

#[derive(Serialize)]
pub enum MessageKind {
    Create,
    Update,
    Delete,
}

#[derive(Serialize)]
pub struct MessageData<'a, T: Serialize + 'a> {
    pub kind: MessageKind,
    pub data: &'a T,
}

impl<'a, T: Serialize + 'a> MessageData<'a, T> {
    pub fn new(kind: MessageKind, data: &'a T) -> Self {
        Self { kind, data }
    }

    pub fn create(data: &'a T) -> Self {
        Self::new(MessageKind::Create, data)
    }

    pub fn update(data: &'a T) -> Self {
        Self::new(MessageKind::Update, data)
    }

    pub fn delete(data: &'a T) -> Self {
        Self::new(MessageKind::Delete, data)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// TODO: Maybe implement TryFrom?
impl<'a, T: Serialize + 'a> From<MessageData<'a, T>> for String {
    fn from(msg: MessageData<T>) -> String {
        serde_json::to_string(&msg).unwrap()
    }
}
