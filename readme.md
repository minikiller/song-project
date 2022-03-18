###
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```
nano ~/.cargo/config
```
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

### axum

https://github.com/tokio-rs/axum    

### sqlx

https://github.com/launchbadge/sqlx

https://github.com/launchbadge/realworld-axum-sqlx/

https://github.com/launchbadge/realworld-axum-sqlx/blob/837713af32fce83d544993a3d8f7664cbe3354b8/src/http/users.rs

output debug info

### cors
> see https://docs.rs/tower-http/latest/tower_http/cors/index.html


### patch

> todo 

### unit test
>

### security 

### postgres setup
> sudo docker run  -d\
    --name song-db \
    -p 5432:5432 \
    -v determined_db:/var/lib/postgresql/data \
    -e POSTGRES_DB=determined \
    -e POSTGRES_PASSWORD=sunlf \
    postgres:10

###
$ cargo install sqlx-cli

sqlx migrate run

1.需求管理 概要设计
2.适合开发团队人数，项目规模
3.如何走集成测试
4.是否和语言相关
5.git也是自研