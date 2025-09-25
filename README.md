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
- 🐳 **Docker対応**: 開発・本番・CI/CD用の最適化されたDockerイメージ
- 🔄 **CI/CD**: GitHub Actionsによる自動テスト・ビルド・デプロイ
- 🧪 **E2Eテスト**: Playwrightによる包括的なブラウザテスト

## セットアップ

### 前提条件
- Rust 1.92.0-nightly
- Node.js 18+
- Google Gemini APIキー（[Google AI Studio](https://aistudio.google.com/)で取得）

### 環境変数の設定

1. **APIキーの設定:**
```bash
# .envファイルを作成
echo "GEMINI_API_KEY=your_gemini_api_key_here" > .env
```

2. **leptos.toml設定の確認:**
```toml
[package]
name = "self-changer"
version = "0.1.0"
edition = "2021"

[build]
site_root = "site"
site_pkg_dir = "pkg"
output_name = "self-changer"

[serve]
reload_port = 3001
hot_reload = true

[env]
LEPTOS_SITE_ROOT = "site"
LEPTOS_SITE_PKG_DIR = "pkg"
LEPTOS_OUTPUT_NAME = "self-changer"
```

### Docker デプロイメント

#### Docker Compose を使用したクイックスタート

1. **リポジトリのクローンと環境設定:**
```bash
git clone https://github.com/aoimaru42/self-changer.git
cd self-changer
echo "GEMINI_API_KEY=your_gemini_api_key_here" > .env
```

2. **Docker Composeで実行:**
```bash
# 開発モード（ホットリロード付き）
docker-compose --profile development up

# 本番モード
docker-compose --profile production up

# ウォッチモード（開発中の自動リロード）
docker-compose --profile watch up
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
echo "GEMINI_API_KEY=your_gemini_api_key_here" > .env
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
│   │       ├── mod.rs     # ページモジュール定義
│   │       └── chat_page.rs # チャットページコンポーネント
├── server/                 # サーバーサイドクレート
│   └── src/
│       └── main.rs        # サーバーエントリーポイント
├── frontend/               # フロントエンドクレート
│   └── src/
│       └── lib.rs         # フロントエンドエントリーポイント
├── common/                 # 共通データ構造
│   └── src/
│       └── lib.rs         # 共通型定義
├── end2end/                # E2Eテスト（Playwright）
│   ├── tests/
│   │   └── example.spec.ts # テストスイート
│   ├── playwright.config.ts # テスト設定
│   └── package.json        # Node.js依存関係
├── style/                  # CSSスタイル
│   └── main.css           # メインスタイルシート
├── public/                 # 静的ファイル
│   └── favicon.ico        # ファビコン
├── .github/                # GitHub Actions設定
│   └── workflows/
│       └── ci.yml         # CI/CDパイプライン
├── Dockerfile              # 本番用Dockerfile
├── Dockerfile.dev          # 開発用Dockerfile
├── Dockerfile.github       # CI/CD用Dockerfile
├── docker-compose.yml      # Docker Compose設定
├── leptos.toml            # Leptos設定ファイル
└── Cargo.toml             # ワークスペース設定
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

### 自動セットアップ（推奨）

```bash
# プロジェクトルートで実行
make setup
# または
./setup.sh
```

### 手動セットアップ

`cargo-leptos`を使用するために必要な追加ツール:

```bash
# Rust nightlyのインストール
rustup toolchain install nightly --allow-downgrade
rustup default nightly

# WebAssemblyコンパイル機能の追加
rustup target add wasm32-unknown-unknown

# 必要なツールのインストール
cargo install cargo-generate
cargo install cargo-leptos --locked

# Node.js依存関係のインストール
cd end2end && npm install
```


## リモートサーバーでのデプロイメント

### Dockerを使用したデプロイ（推奨）

```bash
# リモートサーバーで実行
docker run -d -p 3000:3000 \
  --name self-changer \
  -e GEMINI_API_KEY=your_api_key \
  --restart unless-stopped \
  ghcr.io/aoimaru42/self-changer:latest
```

### バイナリデプロイ

`cargo leptos build --release`を実行後、以下のファイルが必要です:

1. `target/server/release/self-changer` - サーバーバイナリ
2. `target/site/` - 静的ファイルディレクトリ

```bash
# サーバーにファイルをコピー
scp target/server/release/self-changer user@server:/opt/self-changer/
scp -r target/site user@server:/opt/self-changer/

# サーバーで実行
cd /opt/self-changer
./self-changer
```

### 環境変数

```bash
export GEMINI_API_KEY=your_api_key
export LEPTOS_SITE_ADDR=0.0.0.0:3000
```

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

- **開発用**: `Dockerfile.dev` - ホットリロード対応、開発効率重視
- **本番用**: `Dockerfile` - 最適化された本番環境、マルチステージビルド
- **CI/CD用**: `Dockerfile.github` - GitHub Actions専用、マルチアーキテクチャ対応

### CI/CDパイプラインの詳細

#### テストフェーズ
- **Rustテスト**: 単体テストとインテグレーションテスト
- **Clippy**: コード品質とベストプラクティスのチェック
- **Rustfmt**: コードフォーマットの一貫性チェック
- **E2Eテスト**: Playwrightを使用したブラウザテスト

#### ビルドフェーズ
- **Docker Buildx**: マルチアーキテクチャ対応（AMD64/ARM64）
- **GitHub Container Registry**: セキュアなコンテナイメージ配布
- **自動タグ付け**: `latest`、`main`、コミットハッシュタグ

#### セキュリティ
- **Dependabot**: 依存関係の自動更新
- **GitHub Security Advisories**: 脆弱性の自動通知
- **コード署名**: コミットとタグの署名検証

## 📦 Docker Hub / GitHub Container Registry

### GitHub Container Registry (推奨)

```bash
# 最新のイメージをプル
docker pull ghcr.io/aoimaru42/self-changer:latest

# 実行
docker run -d -p 3000:3000 \
  -e GEMINI_API_KEY=your_api_key \
  ghcr.io/aoimaru42/self-changer:latest
```

### Docker Hub

```bash
# 最新のイメージをプル
docker pull aoimaru42/self-changer:latest

# 実行
docker run -d -p 3000:3000 \
  -e GEMINI_API_KEY=your_api_key \
  aoimaru42/self-changer:latest
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

## 🚀 クイックスタート

### 1分で始める

```bash
# リポジトリをクローン
git clone https://github.com/aoimaru42/self-changer.git
cd self-changer

# 環境変数を設定
echo "GEMINI_API_KEY=your_api_key_here" > .env

# Docker Composeで起動
docker-compose --profile development up
```

ブラウザで http://localhost:3000 にアクセスして、AIチャットアプリケーションをお楽しみください！

## 🤝 コントリビューション

プルリクエストやイシューの報告を歓迎します！

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add some amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

## 📄 ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 🙏 謝辞

- [Leptos](https://github.com/leptos-rs/leptos) - 素晴らしいRustフロントエンドフレームワーク
- [Google Gemini API](https://aistudio.google.com/) - AI機能の提供
- [Playwright](https://playwright.dev/) - E2Eテストフレームワーク
