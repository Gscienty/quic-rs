/// MAX_DATA 帧
///
/// 用于流量控制, 通知通信对方可以在整个连接上发送的最大数据量.
///
/// 帧结构如下:
/// MAX_DATA Frame {
///     Type (i) = 0x10,
///     Maximum Data (i),
/// }
pub struct MaxDataFrame {}
