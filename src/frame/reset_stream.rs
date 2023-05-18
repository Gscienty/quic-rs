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
    final_size: u64,
}
