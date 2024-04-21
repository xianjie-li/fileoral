use std::collections::HashMap;

use crate::meta::{Dir, File};

/// 一些需要临时缓存的数据, 在读取后写入缓存, 变更后清理, 缓存存储在内存中, 只在程序生命期有效, 且仅存储在主线程
pub struct Cache {}

pub struct DirCache {}
