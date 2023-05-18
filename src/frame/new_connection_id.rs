/// NEW_CONNECTION_ID 帧结构如下
///
/// 向对方提供可用于在迁移连接时打破关联性的替代连接 ID
///
/// 帧结构如下:
/// NEW_CONNECTION_ID Frame {
///     Type (i) = 0x18,
///     Sequence Number (i),
///     Retire Prior To (i),
///     Length (8),
///     Connection ID (8..160),
///     Stateless Reset Token (128),
/// }
pub struct NewConnectionIDFrame {
    seq: u64,
    retire_prior_to: u64,
    connection_id: Vec<u8>,
    reset_token: [u8; 128],
}
