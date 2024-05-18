use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;
use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct TextPacket {
    pub text_type: u8,
    pub needs_translation: bool,
    pub source_name: String,
    pub parameters: Vec<String>,
    pub xuid: String,
    pub platform_chat_id: String,
}