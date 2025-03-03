# e235718-la-laR

相対パスでのdirectory指定が可能です。

amaneだとsingularityで実行します。localだとrustが動く環境ならそのまま実行できます。

## ls -la
以下のコマンドで実行します。

```
❯ cd la
❯ cargo run <directory>
```

defaultのdirectoryはcurrent directoryなのでdirectoryが指定しなければ、以下の実行と同じ結果になります。
```
❯ cargo run .
```

WARNINGなどを出したくない場合は/dev/nullにリダイレクトしてください。
```
❯ cargo run <directory> 2>/dev/null
```

## ls -laR
以下のコマンドで実行します。
-d <depth>でstackののdepthを拡張できます。defaultのdepthは10です。

```
❯ cd laR
❯ cargo run <directory> -d <depth>
```

また、-dの後に指定したdepthを入れなければエラーになります。

WARNINGなどを出したくない場合は/dev/nullにリダイレクトしてください。
```
❯ cargo run <directory> -d <depth> 2>/dev/null
```
