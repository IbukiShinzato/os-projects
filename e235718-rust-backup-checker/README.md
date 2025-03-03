# e235718-rust-backup-checker

`e235718-rust-backup-checker` は、指定されたソースファイルとそのバックアップファイルが改ざんされていないことを検証するツールです。SHA256ハッシュ値を比較することで、バックアップファイルが正当であるかを確認します。

## 使用方法

1. ソースファイルとバックアップファイルを用意してください。
2. 以下のコマンドでツールを実行します。

```bash
cargo run --release <source_file> <backup_file1> <backup_file2> <backup_file3>
```

実行結果は`logs/`にあります。
