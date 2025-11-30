use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct ConversationId(pub Uuid);

pub enum MessageSender {
    User,
    System,
}

pub struct Message {
    pub id: Uuid,
    pub conversation_id: ConversationId,
    pub sender: MessageSender,
    pub text: String,
    pub created_at: DateTime<Utc>,
}
