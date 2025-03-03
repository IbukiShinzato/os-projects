# e235718-rust-find

## Wrapperd find Command
指定したディレクトリ内でファイルパターンを検索するWrapperです。

### ArgumentsとOptions

```
Singularity> cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/find --help`
Executing as user: "e235718"
Searches for files in a directory with a given pattern

Usage: find --file <pattern> <directory>

Arguments:
  <directory>  Directory to search

Options:
  -f, --file <pattern>  Pattern to match files (e.g., *.c)
  -h, --help            Print help
  -V, --version         Print version
```

## 実行方法
USER環境変数を取得して実行しているので、相対パスでのdirectory指定が可能です。

また、標準エラーは/dev/nullにリダイレクトしています。

```
cargo run -- -f <pattern> <directory>
```
