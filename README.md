## 作业 1 chacha20poly1305

#### 生成密钥

```
cargo run -- text generate -o output/key.txt
```

#### 加密信息

```
cargo run -- text encrypt -k output/key.txt -i assets/juventus.csv -o output/juventus.csv.enc
```

#### 解密信息

```
cargo run -- text decrypt -k output/key.txt -i output/juventus.csv.enc -o output/juventus.csv.dec
```

## 作业 2 显示目录文件

1. `cargo run -- http server`
2. open link `http://localhost:8080`

## 作业 3 jwt

#### 生成 token

```
# 默认参数生成
cargo run -- jwt sign

# 指定 secret key 生成
cargo run -- jwt sign -t <my-secret-key>

# 指定 sub, aud, exp 生成
cargo run -- jwt sign --sub acme --aud device1 --exp 14d
```

#### 验证 token

```
# 验证默认参数生成的token
cargo run -- jwt verify -t <token-value>

# 验证指定 secret key 生成的token
cargo run -- jwt verify -k <my-secret-key> -t <token-value>

# 验证指定 aud 生成的token
cargo run -- jwt verify -a <audience-value> -t <token-value>
```
