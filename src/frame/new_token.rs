use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// NEW_TOKEN 帧
///
/// 由服务器发送, 为客户提供一个令牌.
///
/// 由服务器分配的令牌，客户端用于在未来连接的初始数据包头中发送.
///
/// 帧结构如下:
/// NEW_TOKEN Frame {
///     Type (i) = 0x07,
///     Token Length (i),
///     Token (..),
/// }
pub struct NewTokenFrame {
    /// Token 是客户端可以用于未来 Initial 数据包的不透明 Blob.
    /// 客户端如果 Token 为空的 NEW_TOKEN 帧, 客户端将返回 `FRAME_ENCODING_ERROR` 错误.
    token: Vec<u8>,
}

impl NewTokenFrame {
    /// 构造一个 NEW_TOKEN 帧.
    ///
    /// # Returns
    /// NEW_TOKEN 帧
    pub(crate) fn new() -> Self {
        Self { token: Vec::new() }
    }

    /// 获取 Token
    ///
    /// # Returns
    /// 返回 Token
    #[inline(always)]
    pub(crate) fn get_token(&self) -> &[u8] {
        &self.token
    }

    /// 设置 Token
    ///
    /// # Arguments
    /// `token` - Token
    #[inline(always)]
    pub(crate) fn set_token(&mut self, token: &[u8]) {
        self.token.extend_from_slice(token)
    }
}

impl Serializer for NewTokenFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::NewToken.into()])?;

        payload_size += util::write_varint(self.token.len() as u64, w)?;

        w.write_all(&self.token)?;
        payload_size += self.token.len();

        Ok(payload_size)
    }
}

impl Deserializer for NewTokenFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let token_len = util::read_varint(r)?;
        payload_size += token_len.size;

        self.token.resize(token_len.value as usize, 0);
        r.read_exact(&mut self.token)?;
        payload_size += self.token.len();

        Ok(payload_size)
    }
}
