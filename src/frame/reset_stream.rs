use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// RESET_STREAM 帧
///
/// 用于突然终止发送部分的流.
///
/// 发送 RESET_STREAM 帧之后，发送端将停止在标识的流上传输和重传 STREAM 帧.
/// RESET_STREAM 接收方可以丢弃已经在该流上接收到的任何数据.
///
/// 帧结构如下:
/// RESET_STREAM Frame {
///     Type (i) = 0x04,
///     Stream ID (i),
///     Application Protocol Error Code (i),
///     Final Size (i),
/// }
pub struct ResetStreamFrame {
    stream_id: u64,
    error_code: u64,
    final_size: usize,
}

impl ResetStreamFrame {
    pub fn new() -> Self {
        Self {
            stream_id: 0,
            error_code: 0,
            final_size: 0,
        }
    }
}

impl Serializer for ResetStreamFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::ResetStream.into()])?;

        payload_size += util::write_varint(self.stream_id, w)?;
        payload_size += util::write_varint(self.error_code, w)?;
        payload_size += util::write_varint(self.final_size as u64, w)?;

        Ok(payload_size)
    }
}

impl Deserializer for ResetStreamFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let stream_id = util::read_varint(r)?;
        self.stream_id = stream_id.value;
        payload_size += stream_id.size;

        let error_code = util::read_varint(r)?;
        self.error_code = error_code.value;
        payload_size += error_code.size;

        let final_size = util::read_varint(r)?;
        self.final_size = final_size.value as usize;
        payload_size += final_size.size;

        Ok(payload_size)
    }
}
