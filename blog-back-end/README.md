# blog-back-end
## PostgreSQL
### 参考链接
- 安装：https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-linux/

- 远程连接：https://askubuntu.com/questions/423165/remotely-access-postgresql-database

- 连接 `PostgreSQL`： `sudo -u postgres psql`

- 安装 Web 端 `pgAdmin4`：https://stackoverflow.com/questions/69185413/cant-install-pgadmin4-repository-does-not-have-file

- `DBeaver`：https://dbeaver.io/download/

### 建表语句
``` sql
CREATE TABLE category
(
	id    		serial 						primary key,
    name        VARCHAR(50) 				not null,
    timestamp   timestamp with time zone    not null
);

CREATE TABLE article
(
	id    		serial 						primary key,
	category	VARCHAR(50)					not null,
    name        VARCHAR(50) 				not null,
    timestamp   timestamp with time zone    not null
);

INSERT INTO category(name, timestamp) VALUES ('zk-SNARK', '2022-05-09 16:32:22');
```

### 目录生成
``` sql
SELECT 
    c.name as category, 
    a.name as name, 
    a.timestamp as timestamp 
FROM 
    category as c 
INNER JOIN 
    article as a 
ON 
    c.name = a.category;
```

## Rust
`expect()` 与 `unwrap()`用于处理泛型 `Result<T, E>`，`unwrap()` 使用默认的 `panic!()`，而 `expect()` 可以向 `panic!()` 指定具体的错误描述。

`Option<T>`

``` rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 跨域问题
跨域需要在后端进行配置

### 测试
`cargo test -- --nocapture`

### 时间戳
https://docs.rs/sqlx/0.5.1/sqlx/postgres/types/index.html
