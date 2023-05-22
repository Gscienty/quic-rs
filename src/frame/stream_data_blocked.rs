use crate::{
    attr::{StreamID, StreamIDGetter, StreamIDSetter},
    util,
};

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
pub(crate) struct StreamDataBlockedFrame {
    /// 数据流标识
    stream_id: StreamID,

    /// 数据流流量控制限制
    maximum_data: usize,
}

impl StreamDataBlockedFrame {
    /// 构造一个 STREAM_DATA_BLOCKED 帧
    ///
    /// # Returns
    /// 返回一个 STREAM_DATA_BLOCKED 帧
    pub(crate) fn new() -> Self {
        Self {
            stream_id: 0,
            maximum_data: 0,
        }
    }

    /// 获取数据流最大数据量
    ///
    /// # Returns
    /// 返回数据流的最大数据量
    #[inline(always)]
    pub(crate) const fn get_maximum_data(&self) -> usize {
        self.maximum_data
    }

    /// 设置数据流的最大数据量
    ///
    /// # Arguments
    /// `maximum_data` - 数据流的最大数据量
    #[inline(always)]
    pub(crate) fn set_maximum_data(&mut self, maximum_data: usize) {
        self.maximum_data = maximum_data
    }
}

impl StreamIDGetter for StreamDataBlockedFrame {
    #[inline(always)]
    fn get_stream_id(&self) -> StreamID {
        self.stream_id
    }
}

impl StreamIDSetter for StreamDataBlockedFrame {
    #[inline(always)]
    fn set_stream_id(&mut self, stream_id: StreamID) {
        self.stream_id = stream_id;
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
