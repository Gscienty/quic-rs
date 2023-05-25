use std::io;

use crate::{
    attr::{
        Deserializer, FixedDeserializer, FixedSerializer, PacketNumber, PacketNumberAttr,
        Serializer,
    },
    util,
};

use super::long_header::LongHeader;

/// Initial Packet Header
///
/// 用于携带客户端和服务器发送的第一个 CRYPTO 帧，用以执行密钥交换，并在任一方向上携带 ACK 帧.
///
/// Initial Packet 使用连接和特定版本的密钥进行保护（用于防止伪造初始数据包)
///
/// 服务器响应客户端 Initial Packet 后，发送其第一个 Initial Packet（可以发送多个）
pub(crate) struct InitialHeader {
    header: LongHeader,
    token: Vec<u8>,
    length: usize,
    packet_number: PacketNumber,

    packet_number_len: usize,
}

impl InitialHeader {
    /// 构造 Initial Packet Header
    ///
    /// # Arguments
    /// `packet_number_len` - Packet Number 长度，用于反序列化时读取 Packet Number
    /// # Returns
    /// 返回 Initial Packet Header
    pub(crate) fn new(packet_number_len: usize) -> Self {
        Self {
            header: LongHeader::new(),
            token: Vec::new(),
            length: 0,
            packet_number: 0,

            packet_number_len,
        }
    }
}

impl Serializer for InitialHeader {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        let packet_number_len = self.packet_number.serialize_len()?;
        w.write_all(&[0xc0 | (packet_number_len as u8 - 1)])?;

        payload_size += self.header.write(w)?;

        payload_size += util::write_varint(self.token.len() as u64, w)?;

        w.write_all(&self.token)?;
        payload_size += self.token.len();

        payload_size += util::write_varint(self.length as u64, w)?;

        self.packet_number.write_fixed(packet_number_len, w)?;
        payload_size += packet_number_len;

        Ok(payload_size)
    }
}

impl Deserializer for InitialHeader {
    fn read(&mut self, r: &mut dyn io::Read) -> Result<usize, io::Error> {
        let mut payload_size = self.header.read(r)?;

        let token_length = util::read_varint(r)?;
        payload_size += token_length.size;
        self.token.resize(token_length.value as usize, 0);

        r.read_exact(&mut self.token)?;
        payload_size += token_length.value as usize;

        let length = util::read_varint(r)?;
        payload_size += length.size;
        self.length = length.value as usize;

        self.packet_number.read_fixed(self.packet_number_len, r)?;
        payload_size += self.packet_number_len;

        Ok(payload_size)
    }
}
