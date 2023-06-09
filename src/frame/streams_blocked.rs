use crate::{
    attr::{Deserializer, Serializer},
    util,
};

use super::types::FrameType;

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
    /// 是否是双向流
    bidi_flag: bool,

    /// 允许的最大 Stream 数量.
    maximum_streams: usize,
}

impl StreamsBlockedFrame {
    /// 构造一个 STREAM_BLOCKED 帧
    ///
    /// # Arguments
    /// `bidi_flag`: 是否是双向流
    /// # Returns
    /// 返回一个 STREAM_BLOCKED 帧
    pub fn new(bidi_flag: bool) -> Self {
        Self {
            bidi_flag,
            maximum_streams: 0,
        }
    }

    /// 获取是否是双向流
    ///
    /// # Returns
    /// 返回是否是双向流
    #[inline(always)]
    pub const fn is_bidi(&self) -> bool {
        self.bidi_flag
    }

    /// 设置 STREAM_BLOCKED 是否是双向流
    ///
    /// # Arguments
    /// `bidi_flag` - 是否是双向流
    #[inline(always)]
    pub fn set_bidi(&mut self, bidi_flag: bool) {
        self.bidi_flag = bidi_flag
    }

    /// 获取流数量的限制
    ///
    /// # Returns
    /// 返回流数量的限制
    #[inline(always)]
    pub const fn get_maximum_streams(&self) -> usize {
        self.maximum_streams
    }

    /// 设置流数量的限制
    ///
    /// # Arguments
    /// `maximum_streams` - 流数量的限制
    #[inline(always)]
    pub fn set_maximum_streams(&mut self, maximum_streams: usize) {
        self.maximum_streams = maximum_streams
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
