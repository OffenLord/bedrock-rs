use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct StartGamePacket {
    // target_actor_id: ivar64,
    // target_runtime_id: uvar64,
    // actor_game_type: ivar32,
    // position: Vec3,
    // rotation: Vec2,
    // settings: LevelSettings,
    // level_id: String,
    // level_name: String,
    // template_content_identity: String,
    // trial: bool,
    // movement_settings: PlayerMovementSettings,
    // level_time: u64,
    // block_properties: Vec<>,
    // items: Vec<>,
    // multiplayer_correlation_id: String,
    // item_stack_net_manager: bool,
    // server_version: String,
    // player_property_data: ,
}
