
# 📘 rust-book-study

このリポジトリは、[The Rust Programming Language（Rust Book）](https://doc.rust-lang.org/book/) をベースにRustの文法を体系的に学習し、CLIツールおよびWebアプリケーション開発に繋げるための学習記録用です。

---

## 🎯 学習の目的

- Rustの基本文法（所有権・借用・エラーハンドリング・トレイト等）の理解
- CLIツール開発に必要な設計・実装スキルの獲得
- Webアプリ開発（axum または actix-web）への応用力の習得

---

## 🗓️ 学習スケジュール（Phase 1）

- 期間：2024年4月20日〜6月9日（Rust Book 第1〜13章）
- 毎日1時間、1章または小節ごとに学習
- 進捗は Notion 上のチェックリストで管理

📋 **Notion チェックリスト** → [Rust Book 学習管理ページ](https://www.notion.so/Rust-Book-ToDo-1da3247f91e380c290b8e19eb679c0e0?pvs=4)

---

## 📁 ディレクトリ構成

```
rust-book-study/
├── 01_rust_book/        # 章ごとの演習・練習コード
│   ├── ch01_hello_cargo/
│   ├── ch02_guessing_game/
│   └── ...
├── 02_tips.md           # よく使うコマンド・構文・トリックなど
└── README.md            # このファイル
```

---

## 🔁 学習ログの記録（コミットメッセージルール）

```
date : 04/20
summary : ch01「はじめに」読了。Rustの開発ツールとcargoの概要を理解。
memo : cargo newで作られる構成とビルド手順を確認。
```

- **date**：学習日付（MM/DD形式）
- **summary**：学習内容の要約
- **memo**：補足や所感、気づきなど（任意）

---

## 🔗 関連リポジトリ（CLI / Webアプリ開発）

- [rust-cli-tools](https://github.com/dghnts/rust-cli-tools)
- [rust-web-apps](https://github.com/dghnts/rust-web-apps)
