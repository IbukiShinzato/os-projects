# e235718-rust-memset

rustを用いてmemoryをmemsetしてその際の実行時間を記録するプログラムです。またlibcクレートのmallocを使用しました。

data/malloc_data_macos.csv => ローカル環境での実行結果

data/malloc_data_linux.csv => amaneの環境での実行結果

## memsetの考察

report/report.pdfにありますが、ダウンロードした方が見やすいです。[こちら](https://gitlab.ie.u-ryukyu.ac.jp/os/2024/e235718-rust-memset/-/blob/main/report/report.pdf?ref_type=heads)からダウンロードしてください。

## fragmentationの考察のURL

メモリ断片化に関する考察については[こちら](https://ie.u-ryukyu.ac.jp/~e235718/hugo/post/2025/1/22/1/)をご覧ください。
