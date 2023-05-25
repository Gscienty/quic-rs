use crate::{
    attr::{Deserializer, Serializer, StreamID, StreamIDGetter, StreamIDSetter},
    util,
};

use super::types::FrameType;

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
pub(crate) struct ResetStreamFrame {
    /// Stream 标识
    stream_id: StreamID,

    /// 应用错误码
    ///
    /// 由应用协议自行管理
    error_code: u64,

    /// QUIC 协议中 Stream 的最终大小.
    ///
    /// Stream 消耗的流量控制信用量. 最终大小是发送的最大偏移字节的偏移量加 1,
    /// 如果没有发送任何字节, 则为 0.
    final_size: usize,
}

impl ResetStreamFrame {
    /// 构造一个 RESET_STREAM 帧.
    ///
    /// # Returns
    /// RESET_STREAM 帧
    pub(crate) fn new() -> Self {
        Self {
            stream_id: 0,
            error_code: 0,
            final_size: 0,
        }
    }

    /// 获取 Stream 的最终大小.
    ///
    /// # Returns
    /// 返回 Stream 的最终大小
    #[inline(always)]
    pub(crate) const fn get_final_size(&self) -> usize {
        self.final_size
    }

    /// 设置 Stream 的最终大小.
    ///
    /// # Arguments
    /// `final_size` - Stream 的最终大小
    #[inline(always)]
    pub(crate) fn set_final_size(&mut self, final_size: usize) {
        self.final_size = final_size
    }

    /// 获取应用错误码
    ///
    /// # Returns
    /// 返回应用错误码
    #[inline(always)]
    pub(crate) const fn get_error_code(&self) -> u64 {
        self.error_code
    }

    /// 设置应用错误码
    ///
    /// # Arguments
    /// `error_code` - 应用错误码
    pub(crate) fn set_error_code(&mut self, error_code: u64) {
        self.error_code = error_code
    }
}

impl StreamIDGetter for ResetStreamFrame {
    fn get_stream_id(&self) -> StreamID {
        self.stream_id
    }
}

impl StreamIDSetter for ResetStreamFrame {
    fn set_stream_id(&mut self, stream_id: StreamID) {
        self.stream_id = stream_id
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
