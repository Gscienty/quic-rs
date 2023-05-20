use crate::util;

use super::{
    serialize::{Deserializer, Serializer},
    types::FrameType,
};

/// STOP_SENDING 帧
///
/// 用于请求对方停止在某个流上的传输.
///
/// STOP_SENDING 帧可用于发送状态为 RECV 或者 SIZE_KNOWN 帧.
///
/// 对于还未创建的本地初始化流，如果收到 STOP_SENDING 帧给数据发送方，用于
/// 告知数据发送方已经有多少数据包，必须将其视为类型为 STREAM_STATE_ERROR
/// 的连接错误.
///
/// 对于仅接收数据的数据流，如果收到 STOP_SENDING 帧，必须使用错误
/// STREAM_STATE_ERROR 终止连接.
///
/// 帧结构如下:
/// STOP_SENDING Frame {
///     Type (i) = 0x05,
///     Stream ID (i),
///     Application Protocol Error Code (i),
/// }
pub struct StopSendingFrame {
    stream_id: u64,
    error_code: u64,
}

impl StopSendingFrame {
    pub fn new() -> Self {
        Self {
            stream_id: 0,
            error_code: 0,
        }
    }
}

impl Serializer for StopSendingFrame {
    fn write(&self, w: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let mut payload_size = 1;

        w.write_all(&[FrameType::StopSending.into()])?;

        payload_size += util::write_varint(self.stream_id, w)?;
        payload_size += util::write_varint(self.error_code, w)?;

        Ok(payload_size)
    }
}

impl Deserializer for StopSendingFrame {
    fn read(&mut self, r: &mut dyn std::io::Read) -> Result<usize, std::io::Error> {
        let mut payload_size = 0;

        let stream_id = util::read_varint(r)?;
        self.stream_id = stream_id.value;
        payload_size += stream_id.size;

        let error_code = util::read_varint(r)?;
        self.error_code = error_code.value;
        payload_size += error_code.size;

        Ok(payload_size)
    }
}
