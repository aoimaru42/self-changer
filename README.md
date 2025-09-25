# Self Changer

AI駆動の動的UI変更チャットアプリケーション

Self Changerは、[Leptos](https://github.com/leptos-rs/leptos)フレームワークと[Axum](https://github.com/tokio-rs/axum)を使用して構築された、Google Gemini APIを活用したチャットアプリケーションです。ユーザーとの会話を通じて、リアルタイムでUIのスタイルや要素を動的に変更することができます。

## 主な機能

- 🤖 **AI駆動のUI変更**: Google Gemini APIを使用してユーザーの要求に応じてUIを動的に変更
- 💬 **リアルタイムチャット**: レスポンシブなチャットインターフェース
- 🎨 **動的スタイリング**: CSSプロパティをリアルタイムで変更
- 🧩 **動的要素生成**: 新しいHTML要素を動的に追加
- 🔄 **状態管理**: Leptosのリアクティブシステムによる効率的な状態管理
- 🛡️ **セキュリティ**: CSSサニタイゼーションによる安全なスタイル適用

## セットアップ

### 前提条件
- Rust 1.77+
- Node.js 18+
- Google Gemini APIキー（[Google AI Studio](https://aistudio.google.com/)で取得）

### 環境変数の設定

1. **APIキーの設定:**
```bash
# .envファイルを作成
echo "GEMINI_API_KEY=your_gemini_api_key_here" > .env
```

### Docker デプロイメント

#### Docker Compose を使用したクイックスタート

1. **リポジトリのクローンと環境設定:**
```bash
git clone <your-repo>
cd self-changer
cp .env.example .env
# .envファイルを編集してGEMINI_API_KEYを追加
```

2. **Docker Composeで実行:**
```bash
# 開発モード（ホットリロード付き）
docker-compose up

# 本番モード
docker-compose --profile production up
```

3. **アプリケーションにアクセス:**
- ブラウザで http://localhost:3000 を開く

#### 手動Dockerビルド

```bash
# イメージをビルド
docker build -t self-changer .

# コンテナを実行
docker run -p 3000:3000 -e GEMINI_API_KEY=your_key_here self-changer
```

## ローカル開発

### セットアップ

1. **cargo-leptosのインストール:**
```bash
cargo install cargo-leptos --locked
```

2. **Node.js依存関係のインストール:**
```bash
npm install
```

3. **環境設定:**
```bash
cp .env.example .env
# .envファイルを編集してGEMINI_API_KEYを追加
```

### 開発コマンド

```bash
# 開発サーバーを起動（CSS監視付き）
npm run dev

# または個別に実行:
# ターミナル1: CSS監視
npm run watch:css

# ターミナル2: Leptos監視
cargo leptos watch
```

### 本番ビルド

```bash
# 全体をビルド
npm run build

# または手動で:
npm run build:css
cargo leptos build --release --features ssr
```
## プロジェクト構造

```
self-changer/
├── app/                    # メインアプリケーションクレート
│   ├── src/
│   │   ├── lib.rs         # アプリケーションのエントリーポイント
│   │   ├── api.rs         # Gemini APIとの通信処理
│   │   ├── api_client.rs  # クライアント側API呼び出し
│   │   ├── css_sanitizer.rs # CSSサニタイゼーション
│   │   └── pages/
│   │       └── chat_page.rs # チャットページコンポーネント
├── server/                 # サーバーサイドクレート
│   ├── src/
│   │   ├── main.rs        # サーバーエントリーポイント
│   │   ├── api_handler.rs # APIハンドラー
│   │   └── fileserv.rs    # ファイルサーバー
├── frontend/               # フロントエンドクレート
├── common/                 # 共通データ構造
├── end2end/                # E2Eテスト（Playwright）
├── style/                  # CSSスタイル
└── public/                 # 静的ファイル
```

### 主要コンポーネント

- **ChatPage**: メインのチャットインターフェース
- **API Client**: Gemini APIとの通信を管理
- **CSS Sanitizer**: セキュアなCSSプロパティの適用
- **Dynamic Elements**: リアルタイムでのUI要素の追加・変更

## 使用方法

1. アプリケーションを起動後、ブラウザで http://localhost:3000 にアクセス
2. チャットボックスにメッセージを入力
3. AIがUIの変更要求を理解し、リアルタイムでスタイルや要素を変更

### 使用例

- 「背景を青くして」→ チャットコンテナの背景色を変更
- 「ボタンを追加して」→ 新しいボタン要素を動的に生成
- 「メッセージの色を赤にして」→ 特定のメッセージのスタイルを変更

## テスト

このプロジェクトは[Playwright](https://playwright.dev)を使用してE2Eテストを実行します。

### テストの実行

1. **初回セットアップ:**
```bash
cd end2end
npm install -D playwright @playwright/test
npx playwright install
```

2. **開発中のテスト実行:**
```bash
cargo leptos end-to-end
```

3. **本番ビルドでのテスト実行:**
```bash
cargo leptos end-to-end --release
```

4. **テストレポートの表示:**
```bash
cd end2end
npx playwright show-report
```

## 追加ツールのインストール

`cargo-leptos`を使用するために必要な追加ツール:

1. `rustup toolchain install nightly --allow-downgrade` - Rust nightlyのインストール
2. `rustup default nightly` - nightlyをデフォルトに設定（またはrust-toolchainファイルを使用）
3. `rustup target add wasm32-unknown-unknown` - WebAssemblyコンパイル機能の追加
4. `cargo install cargo-generate` - cargo-generateバイナリのインストール
5. `npm install -g sass` - dart-sassのインストール


## リモートサーバーでのデプロイメント

`cargo leptos build --release`を実行後、以下の最小限のファイルが必要です:

1. `target/server/release`にあるサーバーバイナリ
2. `target/site`ディレクトリとその中のすべてのファイル

これらのファイルをリモートサーバーにコピーします。ディレクトリ構造は以下のようになります:
```text
self-changer
site/
```

以下の環境変数を設定してください（プロジェクトに応じて更新）:
```text
LEPTOS_OUTPUT_NAME="self-changer"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

最後に、サーバーバイナリを実行します。

## 🚀 GitHub Actions CI/CD

このプロジェクトはGitHub Actionsを使用してCI/CDパイプラインを自動化しています。

### 自動化される機能

- **自動テスト**: コードプッシュ時にRustテストとE2Eテストを実行
- **コード品質チェック**: ClippyとRustfmtによるコード品質チェック
- **Dockerイメージビルド**: テスト通過後にDockerイメージを自動ビルド
- **セキュリティ更新**: Dependabotによる依存関係の自動更新

### ワークフロー

1. **プッシュ/PR作成時**:
   - Rustテスト実行
   - Clippyによるコード品質チェック
   - Rustfmtによるフォーマットチェック
   - PlaywrightによるE2Eテスト

2. **mainブランチへのプッシュ時**:
   - 上記テストに加えて
   - Dockerイメージのビルドとプッシュ
   - GitHub Container Registryへの公開

### 使用するDockerイメージ

- **開発用**: `Dockerfile.dev` - ホットリロード対応
- **本番用**: `Dockerfile` - 最適化された本番環境
- **CI/CD用**: `Dockerfile.github` - マルチアーキテクチャ対応

## 📦 Docker Hub / GitHub Container Registry

### GitHub Container Registry (推奨)

```bash
# 最新のイメージをプル
docker pull ghcr.io/[your-username]/self-changer:latest

# 実行
docker run -d -p 3000:3000 \
  -e GEMINI_API_KEY=your_api_key \
  ghcr.io/[your-username]/self-changer:latest
```

### Docker Hub

```bash
# 最新のイメージをプル
docker pull [your-username]/self-changer:latest

# 実行
docker run -d -p 3000:3000 \
  -e GEMINI_API_KEY=your_api_key \
  [your-username]/self-changer:latest
```

## 🔧 環境変数

| 変数名 | 説明 | 必須 | デフォルト値 |
|--------|------|------|-------------|
| `GEMINI_API_KEY` | Google Gemini APIキー | ✅ | - |
| `LEPTOS_SITE_ADDR` | サーバーアドレス | ❌ | `0.0.0.0:3000` |
| `LEPTOS_RELOAD_PORT` | リロードポート | ❌ | `3001` |

## 🛡️ セキュリティ

- **CSS Sanitization**: ユーザー入力のCSSプロパティをサニタイズ
- **Dependabot**: 依存関係の脆弱性を自動チェック
- **GitHub Security Advisories**: セキュリティアドバイザリの自動通知

## 📈 パフォーマンス

- **マルチステージビルド**: Dockerイメージサイズの最適化
- **キャッシュ効率**: GitHub Actionsキャッシュによる高速ビルド
- **マルチアーキテクチャ**: AMD64/ARM64対応

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は[LICENSE](LICENSE)ファイルを参照してください。
