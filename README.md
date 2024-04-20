## 作业 1

### 生成密钥

`cargo run -- text generate -o output/key.txt`

### 加密信息

`cargo run -- text encrypt -k output/key.txt -i assets/juventus.csv -o output/juventus.csv.enc`

### 解密信息

`cargo run -- text decrypt -k output/key.txt -i output/juventus.csv.enc -o output/juventus.csv.dec`

## 作业 2

1. `cargo run -- http server`
2. open link `http://localhost:8080/assets`
