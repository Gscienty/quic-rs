use std::io;

use crate::{
    attr::{ConnectionID, Deserializer, Serializer, Version},
    util,
};

/// 长数据包头
///
/// 用于在建立 1-RTT 密钥之前发送的数据包.
/// 一旦有了 1-RTT 密钥，发送方将切换到使用短数据包头发送数据包.
pub(crate) struct LongHeader {
    /// 版本号
    version: Version,

    /// 目标 Connection ID
    dst: ConnectionID,

    /// 源 Connection ID
    src: ConnectionID,
}

impl LongHeader {
    /// 构造一个 Long Packet Header
    pub(crate) fn new() -> Self {
        Self {
            version: 0x00000000,
            dst: ConnectionID::new(),
            src: ConnectionID::new(),
        }
    }
}

impl Serializer for LongHeader {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let version_bytes = util::to_bigendian_bytes::<_, 4>(self.version);
        w.write_all(&version_bytes)?;
        payload_size += 4;

        payload_size += write_packet_header_connid(self.dst.get_id(), w)?;
        payload_size += write_packet_header_connid(self.src.get_id(), w)?;

        Ok(payload_size)
    }
}

impl Deserializer for LongHeader {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let mut version_bytes = [0u8; 4];
        r.read_exact(&mut version_bytes)?;
        self.version = util::from_bigendian_bytes::<4>(&version_bytes) as Version;
        payload_size += 4;

        let (len, conn_id) = read_packet_header_connid(r)?;
        payload_size += len;
        self.dst = conn_id;

        let (len, conn_id) = read_packet_header_connid(r)?;
        payload_size += len;
        self.src = conn_id;

        Ok(payload_size)
    }
}

/// 向 Write 里写入 Connection ID
///
/// # Arguments
/// `conn_id` - Connection ID
/// `w` - Write
/// # Returns
/// 如果正常写入，则返回写入长度；否则返回 io::Error
#[inline(always)]
fn write_packet_header_connid(conn_id: &[u8], w: &mut dyn io::Write) -> Result<usize, io::Error> {
    let payload_size = 1 + conn_id.len();

    w.write_all(&util::to_bigendian_bytes::<_, 1>(conn_id.len()))?;

    w.write_all(conn_id)?;

    Ok(payload_size)
}

/// 从 Read 中读出 Connection ID
///
/// # Arguments
/// `r`: Reader
/// # Returns
/// 如果正常读出，则返回长度以及 Connection ID；否则返回 io::Error
#[inline(always)]
fn read_packet_header_connid(r: &mut dyn io::Read) -> Result<(usize, ConnectionID), io::Error> {
    let mut payload_size = 0;
    let mut ret = ConnectionID::new();

    let mut len_bytes = [0u8; 1];
    r.read_exact(&mut len_bytes)?;
    let len = util::from_bigendian_bytes::<1>(&len_bytes) as usize;
    payload_size += 1;

    let mut conn_id = Vec::with_capacity(len);
    conn_id.resize(len, 0);
    r.read_exact(&mut conn_id)?;
    payload_size += len;

    ret.set_id(&conn_id);

    Ok((payload_size, ret))
}
