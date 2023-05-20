use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// CRYPTO 帧
///
/// 用于传输加密握手消息. 该帧提供了加密协议的有序字节流.
/// 该帧可以在除 0-RTT 之外的所有数据包类型中发送.
///
/// CRYPTO 帧在功能上与 STREAM 帧完全相同，但 CRYPTO 不带有流标识符,
/// 不受流控制, 不携带可选偏移量、可选长度和流的结尾标记.
///
/// 帧结构如下:
/// CRYPTO Frame {
///     Type (i) = 0x06,
///     Offset (i),
///     Length (i),
///     Crypto Data (..),
/// }
pub struct CryptoFrame {
    offset: usize,
    data: Vec<u8>,
}

impl CryptoFrame {
    pub fn new() -> Self {
        Self {
            offset: 0,
            data: Vec::new(),
        }
    }
}

impl Serializer for CryptoFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::Crypto.into()])?;

        payload_size += util::write_varint(self.offset as u64, w)?;
        payload_size += util::write_varint(self.data.len() as u64, w)?;

        w.write_all(&self.data)?;
        payload_size += self.data.len();

        Ok(payload_size)
    }
}

impl Deserializer for CryptoFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let offset = util::read_varint(r)?;
        self.offset = offset.value as usize;
        payload_size += offset.size;

        let len = util::read_varint(r)?;
        self.data.resize(len.value as usize, 0);
        payload_size += len.size;

        r.read_exact(&mut self.data)?;
        payload_size += self.data.len();

        Ok(payload_size)
    }
}
