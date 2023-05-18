/// CRYPTO 帧
///
/// 用于传输加密握手消息. 该帧提供了加密协议的有序字节流.
/// 该帧可以在除 0-RTT 之外的所有数据包类型中发送.
///
/// CRYPTO 帧在功能上与 STREAM 帧完全相同，但 CRYPTO 不带有流标识符,
/// 不受流控制, 不携带可选偏移量、可选长度和流的结尾标记.
///
/// 帧结构如下:
/// CRYPTO Frame {
///     Type (i) = 0x06,
///     Offset (i),
///     Length (i),
///     Crypto Data (..),
/// }
pub struct CryptoFrame {
    offset: u64,
    data: Vec<u8>,
}
