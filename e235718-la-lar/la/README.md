# e235718-la-laR

## ls -la
以下のコマンドで実行します。

相対パスでのdirectory指定が可能です。

amaneだとsingularityで実行します。localだとrustが動く環境ならそのまま実行できます。
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