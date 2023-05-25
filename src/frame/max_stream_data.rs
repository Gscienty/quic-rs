use crate::{
    attr::{Deserializer, Serializer, StreamID, StreamIDGetter, StreamIDSetter},
    util,
};

use super::types::FrameType;

/// MAX_STREAM_DATA 帧
///
/// 用于流量控制，通知对方可以在流上发送的最大数据量.
///
/// 该帧可用在状态为 RECV 的流上.
/// 当还未创建的本地初始化流接收到该帧，则必须视为 STREAM_STATE_ERROR 错误.
/// 对于仅接收数据的流收到流收到该帧，则必须使用 STREAM_STATE_ERROR 终止连接.
///
/// 帧结构如下:
/// MAX_STREAM_DATA Frame {
///     Type (i) = 0x11,
///     Stream ID (i),
///     Maximum Stream Data (i),
/// }
pub(crate) struct MaxStreamDataFrame {
    /// 流标识
    stream_id: StreamID,

    /// 限定流上发送的最大数据量
    maximum_data: usize,
}

impl MaxStreamDataFrame {
    /// 构造一个 MAX_STREAM_DATA 帧
    ///
    /// # Returns
    /// 返回一个 MAX_STREAM_DATA 帧
    pub(crate) fn new() -> Self {
        Self {
            stream_id: 0,
            maximum_data: 0,
        }
    }

    /// 获取流上发送的最大数据量
    ///
    /// # Returns
    /// 返回流上发送的最大数据量
    #[inline(always)]
    pub(crate) const fn get_maximum_data(&self) -> usize {
        self.maximum_data
    }

    /// 设置流上发送的最大数据量
    ///
    /// # Arguments
    /// `maximum_data` - 流上发送的最大数据量
    #[inline(always)]
    pub(crate) fn set_maximum_data(&mut self, maximum_data: usize) {
        self.maximum_data = maximum_data
    }
}

impl StreamIDGetter for MaxStreamDataFrame {
    fn get_stream_id(&self) -> StreamID {
        self.stream_id
    }
}

impl StreamIDSetter for MaxStreamDataFrame {
    fn set_stream_id(&mut self, stream_id: StreamID) {
        self.stream_id = stream_id
    }
}

impl Serializer for MaxStreamDataFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::MaxStreamData.into()])?;

        payload_size += util::write_varint(self.stream_id, w)?;
        payload_size += util::write_varint(self.maximum_data as u64, w)?;

        Ok(payload_size)
    }
}

impl Deserializer for MaxStreamDataFrame {
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
