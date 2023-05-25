use std::io;

use crate::util;

use super::{FixedDeserializer, FixedSerializer};

/// 数据包编号的范围是 [0, 2^62 - 1].
/// 数据包编号会序列化为1 到 4 个字节，存储在数据包头中.
///
/// 编码数据包受 Header Protection 保护.
pub(crate) type PacketNumber = u64;

pub(crate) trait PacketNumberAttr {
    /// 获取序列化 Packet Number 的长度
    ///
    /// # Returns
    /// 返回 Packet Number 所占的长度
    fn serialize_len(&self) -> Result<usize, io::Error>;
}

impl PacketNumberAttr for PacketNumber {
    fn serialize_len(&self) -> Result<usize, io::Error> {
        let value = *self << 1;

        if value <= 0xff {
            Ok(1)
        } else if value <= 0xffff {
            Ok(2)
        } else if value <= 0xffffff {
            Ok(3)
        } else if value <= 0xffffffff {
            Ok(4)
        } else {
            Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "invalid packet number",
            ))
        }
    }
}

impl FixedSerializer for PacketNumber {
    fn write_fixed(&self, len: usize, w: &mut dyn io::Write) -> Result<(), io::Error> {
        match len {
            1 => w.write_all(&util::to_bigendian_bytes::<_, 1>(*self))?,
            2 => w.write_all(&util::to_bigendian_bytes::<_, 2>(*self))?,
            3 => w.write_all(&util::to_bigendian_bytes::<_, 3>(*self))?,
            4 => w.write_all(&util::to_bigendian_bytes::<_, 4>(*self))?,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "unexcepted packet number length",
                ))
            }
        }
        Ok(())
    }
}

impl FixedDeserializer for PacketNumber {
    fn read_fixed(&mut self, len: usize, r: &mut dyn io::Read) -> Result<(), io::Error> {
        let mut packet_number_buf = [0u8; 4];
        match len {
            1 => {
                r.read_exact(&mut packet_number_buf[..1])?;
                *self = util::from_bigendian_bytes::<1>(&packet_number_buf[..1]);
            }
            2 => {
                r.read_exact(&mut packet_number_buf[..2])?;
                *self = util::from_bigendian_bytes::<2>(&packet_number_buf[..2]);
            }
            3 => {
                r.read_exact(&mut packet_number_buf[..3])?;
                *self = util::from_bigendian_bytes::<3>(&packet_number_buf[..3]);
            }
            4 => {
                r.read_exact(&mut packet_number_buf[..4])?;
                *self = util::from_bigendian_bytes::<4>(&packet_number_buf[..4]);
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "unexcepted packet number length",
                ))
            }
        };
        Ok(())
    }
}
