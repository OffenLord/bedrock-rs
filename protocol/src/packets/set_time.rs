use std::io::Cursor;

use bedrock_core::types::i32be;
use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;
use serialize_derive::{MCProtoDeserialize, MCProtoSerialize};

#[derive(Debug, MCProtoSerialize, MCProtoDeserialize)]
pub struct SetTime {
    pub time: i32be,
}

impl MCProtoSerialize for SetTime {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
        where
            Self: Sized,
    {
        match self.time.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }
}

impl MCProtoDeserialize for SetTime {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
        where
            Self: Sized,
    {
        let time = match u32::proto_deserialize(cursor) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok(Self {
            time,
        })
    }
}
