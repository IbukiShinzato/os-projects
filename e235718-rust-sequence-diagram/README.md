# e235718-rust-sequence-diagram

Rustでの平行処理プログラムです。

引数では;を入れるとrustのプログラムと別のプログラムで分岐されるので、""内に全ての実行コマンドを入れてください。

また、バックグラウンド(&)やパイプ(|)の処理は未完成です。

## 実行方法
```
❯ cargo run "実行コマンド"
```

## 実行結果
```
❯ ls ; ls ..
Cargo.lock  Cargo.toml  README.md  src  target
sequence_diagram

❯ cargo run "ls ; ls .."
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/sequence_diagram 'ls ; ls ..'`

Parent 95562
Child 95563
Cargo.lock	Cargo.toml	README.md	src		target
Child end

Parent 95562
Child 95564
sequence_diagram
Child end
```
