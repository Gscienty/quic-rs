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
    maximum_data: u64,
}
