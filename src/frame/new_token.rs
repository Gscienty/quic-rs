/// NEW_TOKEN 帧
///
/// 由服务器发送, 为客户提供一个令牌.
///
/// 由服务器分配的令牌，客户端用于在未来连接的初始数据包头中发送.
///
/// 帧结构如下:
/// NEW_TOKEN Frame {
///     Type (i) = 0x07,
///     Token Length (i),
///     Token (..),
/// }
pub struct NewTokenFrame {}
