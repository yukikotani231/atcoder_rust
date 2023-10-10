# atcoder_rust

atcoderをrustで解いた答えを残していくリポジトリ

[cargo-compete](https://github.com/qryxip/cargo-compete/tree/master)を使用してテストや提出を行えるようにしている

# コマンド

## 環境構築

```bash
cargo install compete
```
## ログイン

```bash
cargo compete login atcoder
```

## 参加登録

```bash
cargo compete participate atcoder [contest_name]
# contest_name: コンテスト名. 例: abc123
```

## コンテスト用のファイル構成を作成

```bash
cargo compete new [contest_name]
```

以下は↑で作成されたディレクトリ内で実行できる

## テスト実行

```bash
cargo compete test [problem]
# problem: 問題. 例: A, D, G
```

## 提出

```bash
cargo compete submit [problem]
```
