# 蓮華（れんげ）

蓮華は、イベントを管理するためのツールです。
泥の中から綺麗な花を咲かせる「蓮華」から、準備の苦労を綺麗なイベントとして結実させることを
目指すことからこの命名になりました。

## 主な機能（想定）

- アカウント管理
- イベントの作成と管理
- 参加者への招待と通知
- イベントの参加者一覧表示
- イベントの参加者へのメッセージ送信

## システム構成

Rust/AxumでHTTPサーバーを動かし、MaudでHTMLを生成します。画面の部分更新にはhtmx、スタイルにはDaisyUIを利用します。イベントと参加者はSeaORMを通じてPostgreSQLに保存されます。

## 起動方法

1. 環境変数を準備します。

   ```sh
   cp .env.example .env
   source .env
   ```

2. PostgreSQLを起動します。

   ```sh
   docker compose up -d
   ```

3. 初回およびスキーマ更新時にマイグレーションを適用します。

   ```sh
   cargo run -p migration -- up
   ```

4. アプリを起動し、<http://localhost:3000> を開きます。

   ```sh
   cargo run
   ```

## コード整形

Rust標準の`rustfmt`を使用します。未導入の場合は、次のコマンドで追加してください。

```sh
rustup component add rustfmt
```

workspace全体を整形するには、次を実行します。

```sh
cargo fmt --all
```

整形せずにチェックだけ行う場合は、次を実行します。

```sh
cargo fmt --all -- --check
```

## サンプルの機能

- イベントの作成・一覧表示・詳細表示・削除
- 参加者の追加・削除
- 出欠（未定・参加予定・欠席）の変更
- htmxによる参加者とイベントカードの部分更新

## マイグレーション

マイグレーションはアプリ起動時には実行されません。明示的に実行してください。

```sh
cargo run -p migration -- up    # 未適用分を適用
cargo run -p migration -- down  # 直近の適用を戻す
```

新しいマイグレーションは `migration/src` にSeaORM Migrationの実装として追加し、`migration/src/lib.rs` の `Migrator::migrations` に登録します。
