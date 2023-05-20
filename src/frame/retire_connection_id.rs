use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// RETIRE_CONNECTION_ID 帧
///
/// 用于表示不再使用由通信对方发出的连接 ID.
/// 也作为请求通信对方为将来使用发送其他连接 ID.
///
/// 帧结构如下:
/// RETIRE_CONNECTION_ID Frame {
///     Type (i) = 0x19,
///     Sequence Number (i),
/// }
pub struct RetireConnectionIDFrame {
    seq: u64,
}

impl RetireConnectionIDFrame {
    pub fn new() -> Self {
        Self { seq: 0 }
    }
}

impl Serializer for RetireConnectionIDFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::RetireConnectionID.into()])?;

        payload_size += util::write_varint(self.seq, w)?;

        Ok(payload_size)
    }
}

impl Deserializer for RetireConnectionIDFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let seq = util::read_varint(r)?;
        self.seq = seq.value;
        payload_size += seq.size;

        Ok(payload_size)
    }
}
