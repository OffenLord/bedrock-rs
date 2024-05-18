use std::io::Cursor;

use bedrock_core::types::u16le;
use serialize::error::{DeserilizationError, SerilizationError};
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

impl MCProtoSerialize for TextPacket {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
        where
            Self: Sized,
    {
        match self.text_type.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match self.needs_translation.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match self.source_name.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match u16le(self.parameters.len() as u16).proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        for parameters in &self.parameters {
            match parameters.proto_serialize(buf) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        match self.xuid.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match self.platform_chat_id.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }
}

impl MCProtoDeserialize for TextPacket {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
        where
            Self: Sized,
    {
        let text_type = match u8::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let needs_translation = match bool::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let source_name = match String::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let parameters_len = match u16le::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let mut parametrs = vec![];

        for _ in 0..parameters_len.0 {
            match String::proto_deserialize(cursor) {
                Ok(v) => parametrs.push(v),
                Err(e) => return Err(e),
            }
        }

        let xuid = match String::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let platform_chat_id = match String::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok(Self {
            text_type,
            needs_translation,
            source_name,
            parameters,
            xuid,
            platform_chat_id,
        })
    }
}
