use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// CONNECTION_CLOSE 帧
///
/// 用于关闭连接.
///
/// 如果存在未明确关闭的打开流, 则在关闭连接时关闭它们.
///
/// 帧结构如下:
/// CONNECTION_CLOSE Frame {
///     Type (i) = 0x1c..0x1d,
///     Error Code (i),
///     [Frame Type (i)],
///     Reason Phrase Length (i),
///     Reason Phrase (..),
/// }
pub struct ConnectionCloseFrame {
    sys_err: bool,
    error_code: u64,
    frame_type: u64,
    reason: String,
}

impl ConnectionCloseFrame {
    pub fn new(sys_err: bool) -> Self {
        Self {
            sys_err,
            error_code: 0,
            frame_type: 0,
            reason: String::new(),
        }
    }
}

impl Serializer for ConnectionCloseFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::ConnectionClose {
            sys_err: self.sys_err,
        }
        .into()])?;

        payload_size += util::write_varint(self.error_code, w)?;

        if self.sys_err {
            payload_size += util::write_varint(self.frame_type, w)?;
        }

        payload_size += util::write_varint(self.reason.len() as u64, w)?;

        w.write_all(self.reason.as_bytes())?;
        payload_size += self.reason.len();

        Ok(payload_size)
    }
}

impl Deserializer for ConnectionCloseFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let error_code = util::read_varint(r)?;
        self.error_code = error_code.value;
        payload_size += error_code.size;

        if self.sys_err {
            let frame_type = util::read_varint(r)?;
            self.frame_type = frame_type.value;
            payload_size += frame_type.size;
        }

        let len = util::read_varint(r)?;
        let mut reason = Vec::<u8>::with_capacity(len.value as usize);
        reason.resize(len.value as usize, 0);
        payload_size += len.size;

        r.read_exact(&mut reason)?;
        self.reason = String::from_utf8(reason).or(Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid input",
        )))?;
        payload_size += self.reason.len();

        Ok(payload_size)
    }
}
