use std::path::Path;

use tokio::fs;

/// 返回指定目录的[fs::ReadDir], 若不存在则递归创建并返回
pub async fn ensure_dir<T: AsRef<Path>>(path: T) -> Result<fs::ReadDir, std::io::Error> {
    let p = path.as_ref();

    if let Err(err) = fs::create_dir_all(p).await {
        return Err(err);
    }

    fs::read_dir(p).await
}

/// 返回指定目录的[fs::File], 若不存在则递归创建并返回
pub async fn ensure_file<T: AsRef<Path>>(path: T) -> Result<fs::File, std::io::Error> {
    let p: &Path = path.as_ref();

    if let Some(parent_path) = p.parent() {
        let parent_path = parent_path.to_str().unwrap_or("/");

        if parent_path != "/" {
            if let Err(err) = ensure_dir(parent_path).await {
                return Err(err);
            }
        }
    }

    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(p)
        .await?;

    return Ok(file);
}

#[tokio::test]
async fn test_ensure_dir() {
    let a = ensure_dir("./src").await.unwrap();
    let b = ensure_dir("./srca".to_string()).await.unwrap();

    println!("{:#?}", a);
    println!("{:#?}", b);
}

#[tokio::test]
async fn test_ensure_file() {
    let path = Path::new("./ttt/ttt.txt");

    ensure_file(path).await.unwrap();

    println!("{:?}", path);
}
