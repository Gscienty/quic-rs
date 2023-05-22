/// 连接 ID
#[derive(Clone, Copy)]
pub(crate) struct ConnectionID {
    connection_id: [u8; 20],
    len: usize,
}

impl ConnectionID {
    pub(crate) fn new() -> Self {
        Self {
            connection_id: [0; 20],
            len: 0,
        }
    }

    /// 设置 ConnectionID，长度应该在 [1, 20] 区间内
    ///
    /// # Arguments
    /// `connection_id` - ConnectionID
    pub(crate) fn set_id(&mut self, connection_id: &[u8]) {
        let len = connection_id.len();
        assert!(1 <= len && len <= 20);

        self.len = len;
        self.connection_id.copy_from_slice(connection_id);
    }

    /// 获取 Connection ID
    ///
    /// # Returns
    /// 返回 Connection ID
    pub(crate) fn get_id(&self) -> &[u8] {
        return &self.connection_id[..self.len];
    }
}
