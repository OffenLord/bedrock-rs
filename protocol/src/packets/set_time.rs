use bedrock_core::types::i32be;
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;
use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct SetTime {
    pub time: i32be,
}