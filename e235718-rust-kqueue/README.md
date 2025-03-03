# e235718-rust-kqueue

`e235718-rust-kqueue` は、Rust を使用してファイルの監視を行うプログラムです。このプログラムは、`kqueue` システムコールを利用してファイルの変更イベント（書き込み、削除、名前変更など）を監視し、そのイベントが発生した際にログファイルに記録します。

## 機能

- `kqueue` を使ってファイルの変更を監視
- 監視対象ファイルの変更、削除、名前変更などのイベントを検出
- イベント発生時にログファイル（`log/file_watcher.log`）に記録
- イベント内容をコンソールにも表示
- ログファイルおよび監視対象ファイルのディレクトリが存在しない場合、自動的に作成

## 使用方法

### 必要な依存関係

このプログラムは、`nix` ライブラリに依存しています。依存関係は `Cargo.toml` で管理されています。以下のコマンドで依存関係をインストールできます。

```bash
cargo build
```

## テストの結果
``` bash
❯ cargo test
   Compiling kqueue v0.1.0 (/Users/sitz_bnk21/ie-ryukyu/Grade2/Os/7.4/kqueue)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.06s
     Running unittests src/main.rs (target/debug/deps/kqueue-01f5211962ff2d09)

running 1 test
test tests::test_watch_file ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```