use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// 应用状态, 通常在内存占用访问, 但在关闭时会写入到文件内并在重启时加载
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    /// 尚未完成的上传任务, 其中key为文件hash, 在文件树中的格式为:
    /// ```shell
    /// [hash+dir]
    ///     - 1.chunk
    ///     - 2.chunk
    ///     - 3.chunk
    /// ```
    pub incomplete: HashMap<String, Incomplete>,
    /// 所有文件的地址, 以hash为key存储
    pub all_files: HashMap<String, Vec<String>>,
    /// 认证信息, 其中key为文件token
    pub auth: HashMap<String, AuthInfo>,
}

/// 表示一项尚未完成的上传任务
#[derive(Serialize, Deserialize, Debug)]
pub struct Incomplete {
    /// 文件总大小
    pub total: u64,
    /// 每个块的尺寸
    pub chunk_size: u64,
    /// 已上传的块数量
    pub uploaded: u32,
    /// 创建时间
    pub create_at: u64,
    /// 更新时间
    pub update_at: u64,
    /// 文件上传后需要写入的路径 (可能存在同一文件在不同位置上传的情况, 此时文件)
    pub target_path: Vec<String>,
}

/// 已完成的认证信息
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthInfo {
    /// 创建时间
    pub create_at: u64,
    /// 更新时间
    pub update_at: u64,
    /// 失效时间
    pub expire_at: u64,
    /// 认证有效时间, 单位为分钟
    pub expire: u32,
    /// 允许访问的ip/域名白名单, 与配置中的whitelist合并, 支持glob
    pub whitelist: Vec<String>,
}
