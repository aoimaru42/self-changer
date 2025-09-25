# マルチステージビルドを使用してRustアプリケーションをビルド
FROM rust:1.82-slim as builder

# システムの依存関係をインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    perl \
    make \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Node.jsをインストール（Playwrightテスト用）
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# nightlyツールチェーンをインストール
RUN rustup toolchain install nightly

# WebAssemblyターゲットをインストール
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

# nightly版のCargoを使用してcargo-leptosをインストール
RUN cargo +nightly install cargo-leptos --locked

# nightly版のCargoを使用してwasm-bindgen-cliをインストール
RUN cargo +nightly install wasm-bindgen-cli

# 作業ディレクトリを設定
WORKDIR /app

# WebAssemblyビルド用の環境変数を設定
ENV CARGO_TARGET_DIR=/app/target
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_PKG_DIR=/app/site/pkg
ENV LEPTOS_OUTPUT_NAME="self-changer"
ENV LEPTOS_ENV="PROD"

# プロジェクト全体をコピー
COPY . .

# アプリケーションをビルド（本番用）
RUN cargo leptos build --release

# 本番用の軽量イメージ
FROM debian:bookworm-slim

# システムの依存関係をインストール
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 作業ディレクトリを設定
WORKDIR /app

# ビルドされたアプリケーションをコピー
COPY --from=builder /app/target/release/server ./
COPY --from=builder /app/site ./site

# ポート3000を公開
EXPOSE 3000

# 環境変数を設定
ENV LEPTOS_OUTPUT_NAME="self-changer"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT="3001"
ENV LEPTOS_ENV="PROD"

# アプリケーションを実行
CMD ["./server"]
