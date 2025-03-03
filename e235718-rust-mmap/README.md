# e235718-rust-mmap

Rustを用いたmmapによるコピーとその考察

## レポート閲覧方法
以下を実行してください
```
curl -O "https://gitlab.ie.u-ryukyu.ac.jp/os/2024/e235718-rust-mmap/raw/main/report/report.pdf"
open report.pdf
```

## 実行方法
```
cargo run -- <method> <source_file> <target_file>
```

## method一覧

1. **mmap_copy**
   - `mmap` + `memcpy`
   - メモリをマップしてから、`memcpy`でデータをコピー

2. **mmap_write**
   - `mmap` + `write`
   - メモリをマップしてから、`write`でデータをコピー

3. **read_write**
   - `read` + `write`
   - `read`でデータを読み込み、`write`でデータを書き込む

## data一覧

1. **iostat1.log**
   - 2回連続実行の結果
2. **iostat2.log**
   - 1回ずつ実行の結果
