use clap::{Args, Parser};

/** 所有支持的配置 */
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// 上传的文件/缓存等信息的存放地址
    #[arg(name = "root path")]
    pub path: String,
    /// 端口
    #[arg(long, default_value_t = 9010)]
    pub port: u16,
    /// 支持上传的最大文件大小 (MB)
    #[arg(long, default_value_t = 2048)]
    pub max_size: u32,
    /// 最大并发上传数, 超出此数量的请求会进入等待状态
    #[arg(long, default_value_t = 1000)]
    pub max_parallelism: u32,
    /// 单个文件最大并发上传数
    #[arg(long, default_value_t = 6)]
    pub max_single_parallelism: u32,
    /// 认证配置
    #[command(flatten)]
    pub auth: AuthConfig,
}

/** 用于认证的配置, 在cli中通过 auth.xxx 设置 */
#[derive(Args)]
pub struct AuthConfig {
    /// 用于认证的地址, 接收参数为: url?token=xxx, 支持的返回格式为json对象, 可包含 <status: bool>, [expire: num(s)], [whitelist: [string]] 字段
    #[arg(long = "auth-url")]
    pub url: Option<String>,
    /// 默认的认证过期时间 (s), 默认一天
    #[arg(long = "auth-expire", default_value_t = 86400)]
    pub expire: u32,
}
