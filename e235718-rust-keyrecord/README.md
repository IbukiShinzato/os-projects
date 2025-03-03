# e235718-rust-keyrecord

`e235718-rust-keyrecord` は、Rust の `crossterm` ライブラリを使用したキー入力の記録プログラムです。ユーザーがキーボードで入力した内容をファイルに保存し、特定のキー（`q`）が押されるまで入力を受け付け続けます。

## 機能

- ユーザーのキーボード入力をリアルタイムで記録
- `q` キーを押すと入力記録が終了
- 入力された内容をファイルに保存 (`keyboard_input.txt`)
- 入力中の各キーの情報（コード、修飾キー、状態など）もファイルに記録

## 使用方法

### 必要な依存関係

このプログラムは、`crossterm` ライブラリに依存しています。依存関係は `Cargo.toml` で管理されています。以下のコマンドで依存関係をインストールできます。

```bash
cargo build
```

## テストの結果
```bash
❯ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/main.rs (target/debug/deps/keyboard-6d9df77a5a513c65)

running 2 tests
test tests::test_for_ci ... ok
                              test tests::test_record_keys ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.58s
```