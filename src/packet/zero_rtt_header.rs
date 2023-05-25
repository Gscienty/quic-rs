use crate::{
    attr::{
        Deserializer, FixedDeserializer, FixedSerializer, PacketNumber, PacketNumberAttr,
        Serializer,
    },
    util,
};

use super::long_header::LongHeader;

/// 0-RTT Packet Header
///
/// 用于在握手完成之前，作为第一次发送的部分，从客户端向服务器传递“早期”数据.
pub(crate) struct ZeroRTTHeader {
    header: LongHeader,
    length: usize,
    packet_number: PacketNumber,

    packet_number_len: usize,
}

impl ZeroRTTHeader {
    /// 构造 0-RTT Packet Header
    ///
    /// # Arguments
    /// `packet_number_len` - Packet Number 长度，用于反序列化时读取 Packet Number
    /// # Returns
    /// 返回 0-RTT Packet Header
    pub(crate) fn new(packet_number_len: usize) -> Self {
        Self {
            header: LongHeader::new(),
            length: 0,
            packet_number: 0,

            packet_number_len,
        }
    }
}

impl Serializer for ZeroRTTHeader {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        let packet_number_len = self.packet_number.serialize_len()?;
        w.write_all(&[0xd0 | (packet_number_len as u8 - 1)])?;

        payload_size += self.header.write(w)?;

        payload_size += util::write_varint(self.length as u64, w)?;

        self.packet_number.write_fixed(packet_number_len, w)?;
        payload_size += packet_number_len;

        Ok(payload_size)
    }
}

impl Deserializer for ZeroRTTHeader {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = self.header.read(r)?;

        let length = util::read_varint(r)?;
        payload_size += length.size;
        self.length = length.value as usize;

        self.packet_number.read_fixed(self.packet_number_len, r)?;
        payload_size += self.packet_number_len;

        Ok(payload_size)
    }
}
