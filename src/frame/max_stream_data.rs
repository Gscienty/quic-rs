use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

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
pub struct MaxStreamDataFrame {
    stream_id: u64,
    maximum_data: usize,
}

impl MaxStreamDataFrame {
    pub fn new() -> Self {
        Self {
            stream_id: 0,
            maximum_data: 0,
        }
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
