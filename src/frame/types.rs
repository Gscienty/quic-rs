/// QUIC 帧类型枚举定义
pub enum FrameType {
    /// PADDING 帧
    ///
    /// 该帧不承载任何数据，仅仅是用于填充数据包内容.
    /// PADDING 帧可被用于满足 QUIC 初始化数据包的最小长度;
    /// 或是为了抵御流量分析，在受保护的数据包中使用.
    ///
    /// 帧结构如下:
    /// PADDING Frame {
    ///     Type (i) = 0x00,
    /// }
    Padding,

    /// PING 帧
    ///
    /// 使用 QUIC 协议通信的其中一方可以通过 PING 帧检查通信的对方是否仍旧存活;
    /// 或检测是否可通信触达到对方.
    ///
    /// 帧结构如下:
    /// PING Frame {
    ///     Type (i) = 0x01,
    /// }
    Ping,

    /// ACK 帧
    ///
    /// 数据接收方会发送 ACK 帧给数据发送方，用于告知数据发送方已经有多少数据包
    /// 已经被接收并处理.
    ///
    /// ACK 帧可以使用 0x02 或 0x03 两种帧类型字节表示.
    /// 0x02: 不包含 ECM Counts 字段;
    /// 0x03: 需包含 ECM Counts 字段.
    ///
    /// 帧结构如下:
    /// ACK Frame {
    ///     Type (i) = 0x02..0x03,
    ///     Largest Acknowledged (i),
    ///     ACK Delay (i),
    ///     ACK Range Count (i),
    ///     First ACK Range (i),
    ///     ACK Range (..) ...,
    ///     [ECN Counts (...)]
    /// }
    Ack { with_ecm: bool },

    /// RESET_STREAM 帧
    ///
    /// 用于突然终止发送部分的流.
    ///
    /// 帧结构如下:
    /// RESET_STREAM Frame {
    ///     Type (i) = 0x04,
    ///     Stream ID (i),
    ///     Application Protocol Error Code (i),
    ///     Final Size (i),
    /// }
    ResetStream,

    /// STOP_SENDING 帧
    ///
    /// 用于请求对方停止在某个流上的传输.
    ///
    /// 帧结构如下:
    /// STOP_SENDING Frame {
    ///     Type (i) = 0x05,
    ///     Stream ID (i),
    ///     Application Protocol Error Code (i),
    /// }
    StopSending,

    /// CRYPTO 帧
    ///
    /// 用于传输加密握手消息.
    ///
    /// 帧结构如下:
    /// CRYPTO Frame {
    ///     Type (i) = 0x06,
    ///     Offset (i),
    ///     Length (i),
    ///     Crypto Data (..),
    /// }
    Crypto,

    /// NEW_TOKEN 帧
    ///
    /// 由服务器发送, 为客户提供一个令牌.
    ///
    /// 帧结构如下:
    /// NEW_TOKEN Frame {
    ///     Type (i) = 0x07,
    ///     Token Length (i),
    ///     Token (..),
    /// }
    NewToken,

    /// STREAM 帧
    ///
    /// 创建一个流，并携带流数据.
    ///
    /// STREAM 帧中的类型字段采用 0b00001xxx 形式, 帧类型低三位标识如下：
    ///
    /// OFF 位 (0x04) 设置为 1 时，表示存在 Offset 字段.
    /// LEN 位 (0x02) 设置为 1 时，表示存在 Length 字段.
    /// FIN 位 (0x01) 设置为 1 时，表示流结束. 流的最终大小为偏移量和此帧长度之和.
    ///
    /// 帧结构如下:
    /// STREAM Frame {
    ///     Type (i) = 0x08..0x0f,
    ///     Stream ID (i),
    ///     [Offset (i)],
    ///     [Length (i)],
    ///     Stream Data (..),
    /// }
    Stream {
        off_flag: bool,
        len_flag: bool,
        fin_flag: bool,
    },

    /// MAX_DATA 帧
    ///
    /// 用于流量控制, 通知通信对方可以在整个连接上发送的最大数据量.
    ///
    /// 帧结构如下:
    /// MAX_DATA Frame {
    ///     Type (i) = 0x10,
    ///     Maximum Data (i),
    /// }
    MaxData,

    /// MAX_STREAM_DATA 帧
    ///
    /// 用于流量控制，通知对方可以在流上发送的最大数据量
    ///
    /// 帧结构如下:
    /// MAX_STREAM_DATA Frame {
    ///     Type (i) = 0x11,
    ///     Stream ID (i),
    ///     Maximum Stream Data (i),
    /// }
    MaxStreamData,

    /// MAX_STREAM 帧
    ///
    /// 用于通知对方可允许打开的流的最大数量.
    /// 可使用 0x12 和 0x13 两种帧类型字节表示,
    /// 0x12 适用于双向流;
    /// 0x13 适用于单向流.
    ///
    /// 帧结构如下:
    /// MAX_STREAMS Frame {
    ///     Type (i) = 0x12..0x13,
    ///     Maximum Streams (i),
    /// }
    MaxStreams { bidi_flag: bool },

    /// DATA_BLOCKED 帧
    ///
    /// 用于流量控制算法的调整输入.
    ///
    /// 帧结构如下:
    /// DATA_BLOCKED Frame {
    ///     Type (i) = 0x14,
    ///     Maximum Data (i),
    /// }
    DataBlocked,

    /// STREAM_DATA_BLOCKED 帧
    ///
    /// 用于流量控制算法的调整输入，作用在具体的流上.
    ///
    /// 帧结构如下:
    /// STREAM_DATA_BLOCKED Frame {
    ///     Type (i) = 0x15,
    ///     Stream ID (i),
    ///     Maximum Stream Data (i),
    /// }
    StreamDataBlocked,

    /// STREAM_BLOCKED 帧
    ///
    /// 当发送方由于其对等方设置的最大流限制而无法打开流时，发送该帧.
    /// 可使用 0x16 和 0x17 两种帧类型字节表示,
    /// 0x16 表示双向流限制;
    /// 0x17 表示单向流限制.
    ///
    /// 帧结构如下:
    /// STREAM_BLOCKED Frame {
    ///     Type (i) = 0x16..0x17,
    ///     Maximum Streams (i),
    /// }
    StreamsBlocked { bidi_flag: bool },

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
    NewConnectionID,

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
    RetireConnectionID,

    /// PATH_CHALLENGE 帧
    ///
    /// 用于检查与通信对方的可达性, 并在连接迁移期间进行路径验证.
    ///
    /// 帧结构如下:
    /// PATH_CHALLENGE Frame {
    ///     Type (i) = 0x1a,
    ///     Data (64),
    /// }
    PathChallenge,

    /// PATH_RESPONSE 帧
    ///
    /// 用于应答 PATH_CHALLENGE 帧.
    ///
    /// 帧结构如下:
    /// PATH_RESPONSE Frame {
    ///     Type (i) = 0x1a,
    ///     Data (64),
    /// }
    PathResponse,

    /// CONNECTION_CLOSE 帧
    ///
    /// 用于关闭连接
    /// 可使用 0x1c 和 0x1d 两种帧类型字节表示,
    /// 0x1c 用于 QUIC 层级别上发生错误或没有错误时发出信号.
    /// 0x1d 用于通知使用 QUIC 的应用程序发生错误.
    ///
    /// 帧结构如下:
    /// CONNECTION_CLOSE Frame {
    ///     Type (i) = 0x1c..0x1d,
    ///     Error Code (i),
    ///     [Frame Type (i)],
    ///     Reason Phrase Length (i),
    ///     Reason Phrase (..),
    /// }
    ConnectionClose { sys_err: bool },

    /// HANDSHAKE_DONE 帧
    ///
    /// 用于服务端向客户端发出握手确认信号.
    ///
    /// 帧结构如下:
    /// HANDSHAKE_DONE Frame {
    ///     Type (i) = 0x1e,
    /// }
    HandshakeDone,

    /// QUIC 扩展帧
    Extension { type_byte: u8 },
}

impl From<u8> for FrameType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::Padding,
            0x01 => Self::Ping,
            0x02..=0x03 => Self::Ack {
                with_ecm: byte == 0x03,
            },
            0x04 => Self::ResetStream,
            0x05 => Self::StopSending,
            0x06 => Self::Crypto,
            0x07 => Self::NewToken,
            0x08..=0x0f => Self::Stream {
                off_flag: (byte & 0x04) != 0,
                len_flag: (byte & 0x02) != 0,
                fin_flag: (byte & 0x01) != 0,
            },
            0x10 => Self::MaxData,
            0x11 => Self::MaxStreamData,
            0x12..=0x13 => Self::MaxStreams {
                bidi_flag: byte == 0x12,
            },
            0x14 => Self::DataBlocked,
            0x15 => Self::StreamDataBlocked,
            0x16..=0x17 => Self::StreamsBlocked {
                bidi_flag: byte == 0x16,
            },
            0x18 => Self::NewConnectionID,
            0x19 => Self::RetireConnectionID,
            0x1a => Self::PathChallenge,
            0x1b => Self::PathResponse,
            0x1c..=0x1d => Self::ConnectionClose {
                sys_err: byte == 0x1c,
            },
            0x1e => Self::HandshakeDone,
            _ => Self::Extension { type_byte: byte },
        }
    }
}

impl From<FrameType> for u8 {
    fn from(frame: FrameType) -> Self {
        match frame {
            FrameType::Padding => 0x00,
            FrameType::Ping => 0x01,
            FrameType::Ack { with_ecm: false } => 0x02,
            FrameType::Ack { with_ecm: true } => 0x03,
            FrameType::ResetStream => 0x04,
            FrameType::StopSending => 0x05,
            FrameType::Crypto => 0x06,
            FrameType::NewToken => 0x07,
            FrameType::Stream {
                off_flag,
                len_flag,
                fin_flag,
            } => {
                let off_flag = if off_flag { 0x04 } else { 0x00 };
                let len_flag = if len_flag { 0x02 } else { 0x00 };
                let fin_flag = if fin_flag { 0x01 } else { 0x00 };

                0x08 | off_flag | len_flag | fin_flag
            }
            FrameType::MaxData => 0x10,
            FrameType::MaxStreamData => 0x11,
            FrameType::MaxStreams { bidi_flag: true } => 0x12,
            FrameType::MaxStreams { bidi_flag: false } => 0x13,
            FrameType::DataBlocked => 0x14,
            FrameType::StreamDataBlocked => 0x15,
            FrameType::StreamsBlocked { bidi_flag: true } => 0x16,
            FrameType::StreamsBlocked { bidi_flag: false } => 0x17,
            FrameType::NewConnectionID => 0x18,
            FrameType::RetireConnectionID => 0x19,
            FrameType::PathChallenge => 0x1a,
            FrameType::PathResponse => 0x1b,
            FrameType::ConnectionClose { sys_err: true } => 0x1c,
            FrameType::ConnectionClose { sys_err: false } => 0x1d,
            FrameType::HandshakeDone => 0x1e,
            FrameType::Extension { type_byte } => type_byte,
        }
    }
}
