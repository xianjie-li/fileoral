## 基础要点

- hash 即是文件 id, 生成算法为 SHA-256
- no db, 直接用文件数 + 简单的持久化应用数据来进行存储
- 文件存储名: [hash]-[原始名称].[扩展名]



## 实现细节

- 压缩, 整理所有相同的文件, 移动到一个共有目录中, 并在原有位置放一个link
- state在每次改写200ms后写入磁盘, 保持state的精简
- 上传完成后续对比一次sha256



## 技术栈

- 运行时: tokio
- web框架: axum
- 日志: tracing
- json序列化: serde
- 命令行解析: clap
- 错误处理: anyhow



## 设想的接口

以uploader作为前缀


- preflightUpload
  - 入参:  hash, dir?
  - 返回:  path[]?, currentPath,  finishChunk: [1, 5,6,8,10],  missingChunk: [2, 3, 4, 7],  totalChunk: 12
  - 检测指定hash的文件是否存在,  传入dir时, 若文件存在, 会将文件复制一份到dir所在目录并返回path到 currentPath
  - 若文件尚未完成上传, 则返回块信息
- upload
  - 入参: file,  dir
  - 上传文件到指定目录
- sliceUpload
  - fileChunk,  fileHash,   dir,  totalSize,   chunkNum: 10,  currentChunk: 1,
  - 返回: shouldSkip, path
  - 执行分片上传,  返回 path 时表示文件上传完成或已包含现成文件, 可直接使用
- getDir
  - 获取目录信息
  - 返回: dirs, files
- getFile
  - 入参:  path
  - 返回: path, name, size, mimetype, craeteDate, updateDate
- getFileByHash
  - 入参: hash
  - 返回: File[]
  - 过hash获取文件信息(File), 如果文件存在于多个目录, 会获取到多个项
- authCheck
  - 入参: token, dir?
  - 判断是否有权限
- 文件服务
  - /files/dir/file.xx?token
  - 没有权限时返回403和空body
- 杂项
  - Https
  - token读写分离
  - 认证支持域名/ip白名单


### 文件/数据存储

filer 使用的根目录

```shell
- root
  - state.json  # 持久化的状态
  - temp        # 临时文件
    - [hash1]
      file-chunk1
      ...
  - files       # 文件存储主目录
    - dir1
    - dir2

```
