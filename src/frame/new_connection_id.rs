use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// NEW_CONNECTION_ID 帧结构如下
///
/// 向对方提供可用于在迁移连接时打破关联性的替代连接 ID
///
/// 帧结构如下:
/// NEW_CONNECTION_ID Frame {
///     Type (i) = 0x18,
///     Sequence Number (i),
///     Retire Prior To (i),
///     Length (8),
///     Connection ID (8..160),
///     Stateless Reset Token (128),
/// }
pub struct NewConnectionIDFrame {
    seq: u64,
    retire_prior_to: u64,
    connection_id: Vec<u8>,
    reset_token: [u8; 16],
}

impl NewConnectionIDFrame {
    pub fn new() -> Self {
        Self {
            seq: 0,
            retire_prior_to: 0,
            connection_id: Vec::new(),
            reset_token: [0; 16],
        }
    }
}

impl Serializer for NewConnectionIDFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::NewConnectionID.into()])?;

        payload_size += util::write_varint(self.seq, w)?;
        payload_size += util::write_varint(self.retire_prior_to, w)?;

        let len_bytes = util::to_bigendian_bytes::<_, 1>(self.connection_id.len() as u8);
        w.write_all(&len_bytes)?;
        payload_size += len_bytes.len();

        w.write_all(&self.connection_id)?;
        payload_size += self.connection_id.len();

        w.write_all(&self.reset_token)?;
        payload_size += self.reset_token.len();

        Ok(payload_size)
    }
}

impl Deserializer for NewConnectionIDFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let seq = util::read_varint(r)?;
        self.seq = seq.value;
        payload_size += seq.size;

        let retire_prior_to = util::read_varint(r)?;
        self.retire_prior_to = retire_prior_to.value;
        payload_size += retire_prior_to.size;

        let mut len_bytes = [0u8; 1];
        r.read_exact(&mut len_bytes)?;
        let len = util::from_bigendian_bytes::<1>(&len_bytes) as usize;
        self.connection_id.resize(len, 0);
        payload_size += len_bytes.len();

        r.read_exact(&mut self.connection_id)?;
        payload_size += self.connection_id.len();

        r.read_exact(&mut self.reset_token)?;
        payload_size += self.reset_token.len();

        Ok(payload_size)
    }
}
