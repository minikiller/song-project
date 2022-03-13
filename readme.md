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
> docker run \
    --name determined-db \
    --network determined \
    -p 5432:5432 \
    -v determined_db:/var/lib/postgresql/data \
    -e POSTGRES_DB=determined \
    -e POSTGRES_PASSWORD=sunlf \
    postgres:10
