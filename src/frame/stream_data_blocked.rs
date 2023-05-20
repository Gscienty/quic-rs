use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// STREAM_DATA_BLOCKED 帧
///
/// 用于流量控制算法的调整输入，作用在具体的流上.
/// 当发送方由于流级流量控制而无法发送数据时，应发送该帧.
///
/// 帧结构如下:
/// STREAM_DATA_BLOCKED Frame {
///     Type (i) = 0x15,
///     Stream ID (i),
///     Maximum Stream Data (i),
/// }
pub struct StreamDataBlockedFrame {
    stream_id: u64,
    maximum_data: usize,
}

impl StreamDataBlockedFrame {
    pub fn new() -> Self {
        Self {
            stream_id: 0,
            maximum_data: 0,
        }
    }
}

impl Serializer for StreamDataBlockedFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::StreamDataBlocked.into()])?;

        payload_size += util::write_varint(self.stream_id, w)?;
        payload_size += util::write_varint(self.maximum_data as u64, w)?;

        Ok(payload_size)
    }
}

impl Deserializer for StreamDataBlockedFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let stream_id = util::read_varint(r)?;
        self.stream_id = stream_id.value;
        payload_size += stream_id.size;

        let maximum_data = util::read_varint(r)?;
        self.maximum_data = maximum_data.value as usize;
        payload_size += maximum_data.size;

        Ok(payload_size)
    }
}
