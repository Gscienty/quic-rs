use crate::{
    attr::{Deserializer, PacketNumber, Serializer},
    util,
};

use super::types::FrameType;

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
pub(crate) struct RetireConnectionIDFrame {
    /// 表示该数据包编号之后 Connection ID 将不再使用.
    seq: PacketNumber,
}

impl RetireConnectionIDFrame {
    /// 构造一个 RETIRE_CONNECTION_ID 帧
    ///
    /// # Returns
    /// 返回一个 RETIRE_CONNECTION_ID 帧
    pub(crate) fn new() -> Self {
        Self { seq: 0 }
    }

    /// 设置终止使用 Connection ID 的数据包编号
    ///
    /// # Arguments
    /// `seq` - 数据包编号
    #[inline(always)]
    pub(crate) fn set_seq(&mut self, seq: PacketNumber) {
        self.seq = seq;
    }

    /// 获取终止使用 Connection ID 的数据包编号
    ///
    /// # Returns
    /// 返回数据包编号
    pub(crate) const fn get_seq(&self) -> PacketNumber {
        self.seq
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
