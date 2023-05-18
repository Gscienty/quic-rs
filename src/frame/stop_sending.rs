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
pub struct StopSendingFrame {}
