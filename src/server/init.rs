use std::{io::ErrorKind, path::PathBuf};

use axum::body::{Body, Bytes};
use tokio::{
    fs::{self, ReadDir},
    io,
};

use crate::{
    config::Config,
    utils::fs::{ensure_dir, ensure_file},
};

/// 服务运行前, 确保root及相关目录和文件存在, 并将 state.json 读取到内存中

pub async fn init(conf: &Config) {
    // 创建必须的目录
    let state_json_dir = PathBuf::from(&conf.path).join("state.json");
    let temp_dir = PathBuf::from(&conf.path).join("temp");
    let files_dir = PathBuf::from(&conf.path).join("files");

    let state_json = ensure_file(state_json_dir).await.unwrap();
    let temp_dir = ensure_dir(temp_dir).await.unwrap();
    let files_dir = ensure_dir(files_dir).await.unwrap();
}
