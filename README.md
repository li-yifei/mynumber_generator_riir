# MyNumber Generator (rewritten in Rust)

参照元: <https://github.com/yayoimizuha/mynumber_generator>

Ryzen ユーザーのため、CPU 並列処理を行うポート実装です。

出力は zstd で圧縮されます。

## 使い方

rust 1.80 以上でおすすめ

``` shell
cargo run --release -o output.txt.zst -s 1000000 -t 8 -q 1024
```

### オプション

- `-o`: 出力ファイルパス (`-o -`の場合は標準出力)
- `-s`: 並列処理するブロックサイズ
- `-t`: zstd圧縮(マルチスレッド版)のスレッド数 (1か0で無効化)
- `-q`: 書き込みキューサイズ


## パーフォマンス参考

OS: Windows 11
CPU: AMD Ryzen 9 9950X
Storage: PCIE Gen4 x4 SSD
メモリ: 64GB DDR5
コンパイルオプション:

- `target-cpu=native`
- `lto=true`
- `opt-level=3`


| オプション | ブロックサイズ | 圧縮スレッド数 | キューサイズ | 書き出し速度 | メモリ使用量 |
| -------- | ------------ | -------- | -------- | -------- | -------- |
| デフォルト | 1000000 | 8 | 1024 | >100MB/s | <15GB |
