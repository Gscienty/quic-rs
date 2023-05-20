use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// STREAM_BLOCKED 帧
///
/// 当发送方由于其对等方设置的最大流限制而无法打开流时，发送该帧.
///
/// 帧结构如下:
/// STREAM_BLOCKED Frame {
///     Type (i) = 0x16..0x17,
///     Maximum Streams (i),
/// }
pub struct StreamsBlockedFrame {
    bidi_flag: bool,
    maximum_streams: usize,
}

impl StreamsBlockedFrame {
    pub fn new(bidi_flag: bool) -> Self {
        Self {
            bidi_flag,
            maximum_streams: 0,
        }
    }
}

impl Serializer for StreamsBlockedFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::StreamsBlocked {
            bidi_flag: self.bidi_flag,
        }
        .into()])?;

        payload_size += util::write_varint(self.maximum_streams as u64, w)?;

        Ok(payload_size)
    }
}

impl Deserializer for StreamsBlockedFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let maximum_streams = util::read_varint(r)?;
        self.maximum_streams = maximum_streams.value as usize;
        payload_size += maximum_streams.size;

        Ok(payload_size)
    }
}
