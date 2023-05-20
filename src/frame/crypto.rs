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
pub(crate) struct CryptoFrame {
    /// CRYPTO 的数据偏移量
    offset: usize,

    /// CRYPTO 数据
    ///
    /// CRYPTO Stream 上已经传递的最大数据量不能超过 2^62 - 1 字节.
    /// 接收到超过此限制时，应该是 `FRAME_ENCODING_ERROR` 或 `CRYPTO_BUFFER_EXCEEDED` 错误.
    data: Vec<u8>,
}

impl CryptoFrame {
    /// 构造一个 CRYPTO 帧.
    ///
    /// # Returns
    /// CRYPTO 帧
    pub(crate) fn new() -> Self {
        Self {
            offset: 0,
            data: Vec::new(),
        }
    }

    /// 获取 CRYPTO 数据偏移量
    ///
    /// # Returns
    /// CRYPTO 数据偏移量
    #[inline(always)]
    pub(crate) const fn get_offset(&self) -> usize {
        self.offset
    }

    /// 设置 CRYPTO 数据偏移量
    ///
    /// # Arguments
    /// `offset` - 数据偏移量
    #[inline(always)]
    pub(crate) fn set_offset(&mut self, offset: usize) {
        self.offset = offset
    }

    /// 获取 CRYPTO 帧携带的数据
    ///
    /// # Returns
    /// 返回 CRYPTO 帧携带的数据
    #[inline(always)]
    pub(crate) fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// 设置 CRYPTO 帧携带的数据
    ///
    /// # Arguments
    /// `data` - 需要写进 CRYPTO 帧的数据
    #[inline(always)]
    pub(crate) fn set_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
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
