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

Rust/AxumでHTTPサーバーを動かし、AskamaでHTMLを生成します。テンプレートは`apps/renge-web/templates`に配置し、画面の部分更新にはhtmx、スタイルにはDaisyUIを利用します。イベントと参加者はSeaORMを通じてPostgreSQLに保存されます。

## 開発環境の準備

Rust、Docker Compose、Task をインストールしてください。

Task のインストール方法は、<https://taskfile.dev/docs/installation> を参照してください。
macOS で Homebrew を使う場合は、次のコマンドでインストールできます。

```sh
brew install go-task/tap/go-task
```

## 起動方法

Taskfile のセットアップタスクで環境変数ファイルを準備します。

```sh
task setup
```

アプリを起動するには、次のコマンドを実行します。PostgreSQL の起動と、未適用のマイグレーションも自動で実行されます。

```sh
task dev
```

起動後、<http://localhost:3000> を開きます。

## ログ

HTTPリクエストの受信時と完了時に、`request_id`、送信元IP、HTTPメソッド、パス、Host、User-Agent、レスポンスステータス、処理時間を出力します。`request_id` はUUIDとして生成され、レスポンスの `X-Request-Id` ヘッダーにも返します。クエリ文字列、Cookie、認証ヘッダー、リクエスト本文、レスポンス本文は出力しません。

| 環境変数 | 既定値 | 内容 |
| --- | --- | --- |
| `LOG_FORMAT` | `pretty` | 開発向けの可読形式は `pretty`、ログ収集基盤向けのJSON形式は `json` |
| `TRUST_PROXY_HEADERS` | `false` | `true` の場合、`X-Forwarded-For` の先頭IPを送信元IPとして使用 |

運用環境では `LOG_FORMAT=json` を設定してください。CaddyまたはNginxを前段に置く場合は、アプリケーションへの直接アクセスを禁止し、プロキシが `X-Forwarded-For` をクライアントIPで上書きする構成にしたうえで、`TRUST_PROXY_HEADERS=true` を設定してください。

## Task 一覧

| コマンド | 内容 |
| --- | --- |
| `task setup` | `.env.example` から `.env` を作成 |
| `task db:up` | PostgreSQL を起動 |
| `task db:down` | PostgreSQL を停止 |
| `task db:migrate` | 未適用のマイグレーションを適用 |
| `task db:migrate:down` | 直近のマイグレーションを戻す |
| `task db:generate-entity` | PostgreSQL のスキーマから SeaORM Entity を生成 |
| `task dev` | DB とマイグレーションを準備してアプリを起動 |
| `task fmt` | workspace 全体を整形 |
| `task fmt:check` | format 済みか確認 |
| `task check` | コンパイルチェック |
| `task test` | テストを実行 |
| `task ci` | CI 用の format・check・test |

Taskfile は `.env` を自動で読み込むため、`source .env` は不要です。

## コード整形

Rust標準の`rustfmt`を使用します。未導入の場合は、次のコマンドで追加してください。

```sh
rustup component add rustfmt
```

workspace全体を整形するには、次を実行します。

```sh
task fmt
```

整形せずにチェックだけ行う場合は、次を実行します。

```sh
task fmt:check
```

## サンプルの機能

- イベントの作成・一覧表示・詳細表示・削除
- 参加者の追加・削除
- 出欠（未定・参加予定・欠席）の変更
- htmxによる参加者とイベントカードの部分更新

## マイグレーション

アプリ本体は起動時にマイグレーションを実行しません。`task dev` ではアプリ起動前に未適用のマイグレーションを適用します。手動で実行する場合は、次のコマンドを使用してください。

```sh
task db:migrate       # 未適用分を適用
task db:migrate:down  # 直近の適用を戻す
```

新しいマイグレーションは `infra/postgres/migration/src` にSeaORM Migrationの実装として追加し、`infra/postgres/migration/src/lib.rs` の `Migrator::migrations` に登録します。

## Entity の生成

`sea-orm-cli` をインストールしていない場合は、SeaORM の依存バージョンに合わせて追加します。

```sh
cargo install sea-orm-cli --version 1.1.20
```

PostgreSQL を起動してマイグレーションを適用した後、DB スキーマから Entity を生成します。生成された `event_handlers` / `participant_handlers` などのファイルは `infra/postgres/renge-orm/src/orm` に保存され、Git で管理します。

```sh
task db:generate-entity
```

`task dev` は Entity を自動再生成しません。マイグレーションや DB スキーマを変更した場合は、必要に応じて `task db:generate-entity` を実行してください。
