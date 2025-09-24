# GitHub Actions Setup Guide

## Required GitHub Secrets

GitHubリポジトリのSettings > Secrets and variables > Actionsで以下のシークレットを設定してください：

### 1. GEMINI_API_KEY
- **説明**: Google Gemini APIキー
- **取得方法**: [Google AI Studio](https://aistudio.google.com/)でAPIキーを生成
- **設定場所**: Repository secrets

### 2. GITHUB_TOKEN (自動設定済み)
- **説明**: GitHub Container Registryへのプッシュ用
- **設定**: 自動的に設定されるため、手動設定不要

## GitHub Actions ワークフロー

このリポジトリには以下のGitHub Actionsワークフローが設定されています：

### 1. CI/CD Pipeline (`.github/workflows/ci-cd.yml`)
- **トリガー**: mainブランチへのプッシュ、プルリクエスト、リリース
- **機能**:
  - E2Eテストの実行
  - Dockerイメージのビルドとプッシュ
  - 本番環境へのデプロイ（設定時）

### 2. Security and Quality Checks (`.github/workflows/security-quality.yml`)
- **トリガー**: mainブランチへのプッシュ、プルリクエスト、週次スケジュール
- **機能**:
  - Trivyによるセキュリティスキャン
  - Rust clippyによるコード品質チェック
  - rustfmtによるコードフォーマットチェック

## Docker イメージ

GitHub Container Registry (ghcr.io) に以下のタグでイメージがプッシュされます：

- `ghcr.io/yourusername/self-changer:main` - 最新のmainブランチ
- `ghcr.io/yourusername/self-changer:sha-xxxxxxx` - 特定のコミット
- `ghcr.io/yourusername/self-changer:v1.0.0` - リリースタグ

## ローカルでのDocker実行

```bash
# 環境変数を設定
export GEMINI_API_KEY=your_api_key_here

# Docker Composeで実行
docker-compose up

# 開発モード（ホットリロード）
docker-compose --profile dev up
```

## トラブルシューティング

### GitHub Actionsが失敗する場合
1. `GEMINI_API_KEY`シークレットが正しく設定されているか確認
2. ワークフローファイルの構文エラーがないか確認
3. Actions タブでログを確認

### Dockerビルドが失敗する場合
1. Dockerfileの構文を確認
2. 必要な依存関係がインストールされているか確認
3. ビルドログでエラーメッセージを確認
