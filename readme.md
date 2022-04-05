### 安装 rust
```bsh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```
### 修改镜像地址
nano ~/.cargo/config

```text
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

### postgres setup
> sudo docker run  -d\
    --name song-db \
    -p 5432:5432 \
    -v determined_db:/var/lib/postgresql/data \
    -e POSTGRES_DB=determined \
    -e POSTGRES_PASSWORD=sunlf \
    postgres:10

### 安装 sqlx

```
$ cargo install sqlx-cli

sqlx migrate run
```

####  数据库表结构说明

| 名称  | 类型  | 说明 |
| :------------ |:---------------|:-----------|
| id        | i32 | 唯一编号 |
| content   | String        |   长度1024，存储错误文件内容 |
| device_id | i32       |    医疗设备编号 |
| time      | datetime  |    记录创建时间 |

### 源码安装并运行

```
git clone https://github.com/minikiller/song-project
cd song-project
cargo build --release
cargo run --release
```

### 数据访问说明
#### 提交数据到服务器
```json
POST http://150.158.141.195:3000/bodys HTTP/1.1
content-type: application/json

{
    "id":101,
    "device_id":5,
    "content": "sample"
}
```

#### 查看数据

```json
GET http://150.158.141.195:3000/bodys HTTP/1.1
```

#### 修改数据
```json
PATCH  http://localhost:3000/bodys/2 HTTP/1.1
content-type: application/json

{
    "id":3,
    "device_id":2,
    "content": "sample123456"
}
```
#### 删除数据

```json
DELETE  http://localhost:3000/bodys/1 HTTP/1.1
```

### 授权访问地址，获得授权jwt

```
POST http://150.158.141.195:3000/authorize HTTP/1.1
content-type: application/json

{
    "client_id": "foo",
    "client_secret": "bar"
}
```

> 返回信息如下：

```json
"access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuYW1lIjoiaGVsbG8iLCJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjIwMDAwMDAwMDB9.HlvHZEu5dTXyzna6UxH4nAwW8YumZdZSq9t_Mdz3h3A",
  "token_type": "Bearer"
```

### 安全访问的地址
```json
GET http://localhost:3000/protected

Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuYW1lIjoiaGVsbG8iLCJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjIwMDAwMDAwMDB9.HlvHZEu5dTXyzna6UxH4nAwW8YumZdZSq9t_Mdz3h3A

```

### 日志输出

修改.env 文件，调整如下内容，就可以控制日志的显示级别：
```text
RUST_LOG=song_project=debug,tower_http=debug,sqlx=info
```