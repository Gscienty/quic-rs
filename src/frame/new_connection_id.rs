use crate::{
    attr::{ConnectionID, PacketNumber},
    util,
};

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
pub(crate) struct NewConnectionIDFrame {
    /// 发送方分配给 Connection ID 的序列号
    seq: PacketNumber,

    /// 表示哪些 Connection ID 应该被废弃
    retire_prior_to: PacketNumber,

    /// 新的 Connection ID
    connection_id: ConnectionID,

    /// 关联的 Connection ID 被使用时，将用于无状态重置
    ///
    /// 当一个端点无法访问连接的状态时，无状态重置可以作为最后的选择.
    /// 例如：如果出现崩溃或者停机时，对等方可能会继续向无法正常维护的端点发送数据.
    /// 这种情况下，端点可以针对收到的无法与活动连接关联的数据包发送“无状态重置”.
    ///
    /// 无状态重置不适用于指示活动连接中的错误.
    ///
    /// 为支持无状态重置过程，一个端点会生成一个无状态重置令牌(16字节的随机数).
    /// 如果对等方随后收到一个以该无状态重置令牌结尾的 UDP 数据包，则对等方将立即结束连接.
    ///
    /// 无状态重置令牌是特定于 Connection ID 的.
    reset_token: [u8; 16],
}

impl NewConnectionIDFrame {
    /// 构造一个 NEW_CONNECTION_ID 帧
    ///
    /// # Returns
    /// 返回一个 NEW_CONNECTION_ID 帧
    pub fn new() -> Self {
        Self {
            seq: 0,
            retire_prior_to: 0,
            connection_id: ConnectionID::new(),
            reset_token: [0; 16],
        }
    }

    /// 获取分配给 Connection ID 的序列号
    ///
    /// # Returns
    /// 返回一个序列号
    #[inline(always)]
    pub(crate) const fn get_seq(&self) -> PacketNumber {
        self.seq
    }

    /// 设置分配给 Connection ID 的序列号
    ///
    /// # Arguments
    /// `seq` - 序列号
    #[inline(always)]
    pub(crate) fn set_seq(&mut self, seq: PacketNumber) {
        self.seq = seq
    }

    /// 获取被废弃 Connection ID 的序列号
    ///
    /// # Returns
    /// 返回一个序列号
    #[inline(always)]
    pub(crate) const fn get_retire_prior_to(&self) -> PacketNumber {
        self.retire_prior_to
    }

    /// 设置被废弃 Connection ID 的序列号
    ///
    /// # Arguments
    /// `seq` - 序列号
    #[inline(always)]
    pub(crate) fn set_retire_prior_to(&mut self, retire_prior_to: PacketNumber) {
        self.retire_prior_to = retire_prior_to
    }

    /// 获取 Connection ID
    ///
    /// # Returns
    /// 返回一个 Connection ID
    #[inline(always)]
    pub(crate) const fn get_connection_id(&self) -> ConnectionID {
        self.connection_id
    }

    /// 设置 Connection ID
    ///
    /// # Arguments
    /// `connection_id` - Connection ID
    #[inline(always)]
    pub(crate) fn set_connection_id(&mut self, connection_id: ConnectionID) {
        self.connection_id = connection_id
    }

    /// 获取无状态重置令牌
    ///
    /// # Returns
    /// 返回一个无状态重置令牌
    #[inline(always)]
    pub(crate) const fn get_reset_token(&self) -> &[u8] {
        &self.reset_token
    }

    /// 设置无状态重置令牌
    ///
    /// # Arguments
    /// `reset_token` - 无状态重置令牌
    #[inline(always)]
    pub(crate) fn set_reset_token(&mut self, reset_token: &[u8]) {
        self.reset_token.copy_from_slice(reset_token)
    }
}

impl Serializer for NewConnectionIDFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::NewConnectionID.into()])?;

        payload_size += util::write_varint(self.seq, w)?;
        payload_size += util::write_varint(self.retire_prior_to, w)?;

        let conn_id = self.connection_id.get_id();

        let len_bytes = util::to_bigendian_bytes::<_, 1>(conn_id.len() as u8);
        w.write_all(&len_bytes)?;
        payload_size += len_bytes.len();

        w.write_all(conn_id)?;
        payload_size += conn_id.len();

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
        payload_size += len_bytes.len();

        let mut conn_id = Vec::with_capacity(len);
        conn_id.resize(len, 0);
        r.read_exact(&mut conn_id)?;
        self.connection_id.set_id(&conn_id);
        payload_size += conn_id.len();

        r.read_exact(&mut self.reset_token)?;
        payload_size += self.reset_token.len();

        Ok(payload_size)
    }
}
