/// RETIRE_CONNECTION_ID 帧
///
/// 用于表示不再使用由通信对方发出的连接 ID.
/// 也作为请求通信对方为将来使用发送其他连接 ID.
///
/// 帧结构如下:
/// RETIRE_CONNECTION_ID Frame {
///     Type (i) = 0x19,
///     Sequence Number (i),
/// }
pub struct RetireConnectionIDFrame {
    seq: u64,
}
