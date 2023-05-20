use std::io::{self, Read, Write};

/// QUIC 帧序列化特征
///
/// 具备 Serializer 特征的 QUIC 帧，具备把自身的数据转化为网络比特流
pub(crate) trait Serializer {
    /// 将自身字段信息序列化为网络比特流
    ///
    /// # Arguments
    /// `w` - 具备 io::Write 特征的一个实现
    /// # Returns
    /// 若序列化成功，则返回写入的数据长度;
    /// 若序列化的过程中出现错误，则返回 io::Error.
    fn write(&self, w: &mut dyn Write) -> Result<usize, io::Error>;
}

/// QUIC 帧反序列化特征
///
/// 具备 Deserializer 特征的 QUIC 帧，具备将网络流转化为自身字段值的能力
pub(crate) trait Deserializer {
    /// 网络比特流中的信息转化为自身字段信息
    ///
    /// # Arguments
    /// `w` - 具备 io::Read 特征的一个实现
    /// # Returns
    /// 若反序列化成功，则返回读出的数据长度;
    /// 若反序列化失败，则返回对应 io::Error.
    fn read(&mut self, r: &mut dyn Read) -> Result<usize, io::Error>;
}
