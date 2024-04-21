/// 表示一个文件
pub struct File {
    /// 文件路径
    pub path: &'static str,
    /// 文件尺寸
    pub size: u64,
    /// 文件meta类型
    pub meta_type: &'static str,
    /// 创建时间
    pub create_at: u64,
    /// 更新时间
    pub update_at: u64,
}

/// 表示一个目录
pub struct Dir {
    /// 文件列表
    pub files: Vec<File>,
    /// 目录列表
    pub dirs: Vec<&'static str>,
    /// 创建时间
    pub create_at: u64,
    /// 更新时间
    pub update_at: u64,
}
