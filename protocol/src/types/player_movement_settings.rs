use bedrock_core::types::i32be;
use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct PlayerMovementSettings {
    movement_type: i32be,
    rewind_history_size: i32be,
    server_authoritative_block_breaking: bool,
}
