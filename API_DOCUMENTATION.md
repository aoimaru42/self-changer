# API ドキュメント

このドキュメントでは、Self ChangerアプリケーションのすべてのパブリックAPI、関数、コンポーネントについて説明します。

## 目次

1. [共通データ構造](#共通データ構造)
2. [API関数](#api関数)
3. [APIクライアント](#apiクライアント)
4. [CSSサニタイザー](#cssサニタイザー)
5. [UIコンポーネント](#uiコンポーネント)
6. [サーバーエントリーポイント](#サーバーエントリーポイント)
7. [フロントエンドエントリーポイント](#フロントエンドエントリーポイント)

---

## 共通データ構造

### `SendMessageRequest`

メッセージ送信APIへのリクエストボディを表す構造体です。

**定義場所:** `common/src/lib.rs`

```rust
pub struct SendMessageRequest {
    pub text: String,
    pub messages: Vec<MessageInfo>,
}
```

**フィールド:**
- `text: String` - ユーザーが送信するメッセージテキスト
- `messages: Vec<MessageInfo>` - 現在のメッセージ履歴（コンテキスト提供用）

**使用例:**
```rust
use common::SendMessageRequest;

let request = SendMessageRequest {
    text: "背景を青くして".to_string(),
    messages: vec![
        MessageInfo {
            id: 0,
            is_user: false,
            text: "こんにちは！".to_string(),
        },
        MessageInfo {
            id: 1,
            is_user: true,
            text: "背景を青くして".to_string(),
        },
    ],
};
```

---

### `SendMessageResponse`

メッセージ送信APIからのレスポンスを表す構造体です。

**定義場所:** `common/src/lib.rs`

```rust
pub struct SendMessageResponse {
    pub success: bool,
    pub message: String,
    pub chat_container_styles: Option<String>,
    pub change_style_elements: Option<Vec<StyleUpdate>>,
    pub new_elements: Option<Vec<DynamicElementData>>,
}
```

**フィールド:**
- `success: bool` - API呼び出しが成功したかどうか
- `message: String` - AIからの応答メッセージ
- `chat_container_styles: Option<String>` - チャットコンテナ全体に適用するCSSプロパティ文字列（例: `"background-color: #3b82f6;"`）
- `change_style_elements: Option<Vec<StyleUpdate>>` - 既存のメッセージ要素に適用するスタイル変更のリスト
- `new_elements: Option<Vec<DynamicElementData>>` - 新しく追加する動的要素のリスト

**使用例:**
```rust
use common::SendMessageResponse;

let response = SendMessageResponse {
    success: true,
    message: "背景を青に変更しました".to_string(),
    chat_container_styles: Some("background-color: #3b82f6;".to_string()),
    change_style_elements: None,
    new_elements: None,
};
```

---

### `MessageInfo`

メッセージ情報を表す構造体です。AIにコンテキストを提供するために使用されます。

**定義場所:** `common/src/lib.rs`

```rust
pub struct MessageInfo {
    pub id: usize,
    pub is_user: bool,
    pub text: String,
}
```

**フィールド:**
- `id: usize` - メッセージの一意なID（0から始まる連番）
- `is_user: bool` - ユーザーメッセージの場合は`true`、AIメッセージの場合は`false`
- `text: String` - メッセージのテキスト内容

**使用例:**
```rust
use common::MessageInfo;

let message = MessageInfo {
    id: 0,
    is_user: false,
    text: "こんにちは！".to_string(),
};
```

---

### `DynamicElementData`

動的に生成する要素のデータを表現する汎用的な構造体です。

**定義場所:** `common/src/lib.rs`

```rust
pub struct DynamicElementData {
    pub id: usize,
    pub tag: String,
    pub text: Option<String>,
    pub styles: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
}
```

**フィールド:**
- `id: usize` - 要素の一意なID（通常は0から開始）
- `tag: String` - HTMLタグ名（`"button"`, `"img"`, `"a"`, `"p"`, `"div"`など）
- `text: Option<String>` - 要素に表示するテキスト（タグによっては`None`の場合あり）
- `styles: Option<String>` - CSSプロパティ文字列（例: `"background-color: #007bff; color: white; padding: 10px 20px;"`）
- `attributes: Option<HashMap<String, String>>` - HTML属性のマップ（例: `{"href": "https://example.com", "target": "_blank"}`）

**サポートされているタグ:**
- `button`: ボタン要素
- `img`: 画像要素（`attributes`に`src`と`alt`を指定）
- `a`: リンク要素（`attributes`に`href`と`target`を指定）
- `p`: 段落要素
- `div`: 汎用コンテナ要素
- `span`: インライン要素
- `h1`, `h2`, `h3`, `h4`, `h5`, `h6`: 見出し要素
- `hr`: 区切り線要素
- `br`: 改行要素

**使用例:**
```rust
use common::DynamicElementData;
use std::collections::HashMap;

// ボタン要素の作成
let button = DynamicElementData {
    id: 0,
    tag: "button".to_string(),
    text: Some("クリックしてください".to_string()),
    styles: Some("background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer;".to_string()),
    attributes: None,
};

// 画像要素の作成
let mut img_attrs = HashMap::new();
img_attrs.insert("src".to_string(), "https://picsum.photos/300/200".to_string());
img_attrs.insert("alt".to_string(), "サンプル画像".to_string());

let image = DynamicElementData {
    id: 1,
    tag: "img".to_string(),
    text: None,
    styles: Some("max-width: 100%; height: auto; border-radius: 8px; margin: 10px 0; display: block;".to_string()),
    attributes: Some(img_attrs),
};

// リンク要素の作成
let mut link_attrs = HashMap::new();
link_attrs.insert("href".to_string(), "https://example.com".to_string());
link_attrs.insert("target".to_string(), "_blank".to_string());

let link = DynamicElementData {
    id: 2,
    tag: "a".to_string(),
    text: Some("こちらをクリック".to_string()),
    styles: Some("color: #007bff; text-decoration: underline; font-weight: bold;".to_string()),
    attributes: Some(link_attrs),
};
```

---

### `StyleUpdate`

既存の要素のスタイルを変更するための構造体です。

**定義場所:** `common/src/lib.rs`

```rust
pub struct StyleUpdate {
    pub id: usize,
    pub styles: String,
}
```

**フィールド:**
- `id: usize` - スタイルを変更するメッセージ要素のID
- `styles: String` - 適用するCSSプロパティ文字列（例: `"background-color: #3b82f6; color: white;"`）

**使用例:**
```rust
use common::StyleUpdate;

// 2番目のメッセージ要素を青にする
let style_update = StyleUpdate {
    id: 2,
    styles: "background-color: #3b82f6; color: white;".to_string(),
};

// 複数の要素に同じスタイルを適用（文字を太字にする）
let style_updates = vec![
    StyleUpdate {
        id: 0,
        styles: "font-weight: bold;".to_string(),
    },
    StyleUpdate {
        id: 1,
        styles: "font-weight: bold;".to_string(),
    },
];
```

---

## API関数

### `send_message`

サーバーサイドで実行される関数で、Google Gemini APIを使用してメッセージを処理し、UI変更指示を含むJSONレスポンスを返します。

**定義場所:** `app/src/api.rs`

**シグネチャ:**
```rust
#[server]
pub async fn send_message(_req: SendMessageRequest) -> Result<SendMessageResponse, ServerFnError>
```

**パラメータ:**
- `_req: SendMessageRequest` - メッセージ送信リクエスト

**戻り値:**
- `Result<SendMessageResponse, ServerFnError>` - 成功時は`SendMessageResponse`、失敗時は`ServerFnError`

**動作:**
1. 環境変数`GEMINI_API_KEY`からAPIキーを取得
2. Google Gemini APIクライアントを初期化（`gemini-2.0-flash`モデルを使用）
3. プロンプトテンプレートを作成し、ユーザーリクエストとメッセージ履歴を埋め込む
4. AIモデルにメッセージを送信
5. AIからの応答テキストを取得
6. JSONを抽出し、CSSサニタイザーでサニタイズ
7. `SendMessageResponse`を構築して返す

**エラーハンドリング:**
- APIキーが見つからない場合: `ServerFnError::new("APIキーが見つかりません: {error}")`
- クライアント初期化エラー: `ServerFnError::new("クライアント初期化エラー: {error}")`
- API呼び出しエラー: `ServerFnError::new("API呼び出しエラー: {error}")`
- JSON解析エラー: フォールバックとして`success: false`のレスポンスを返す

**使用例:**
```rust
use app::api::send_message;
use common::{SendMessageRequest, MessageInfo};

#[tokio::main]
async fn main() {
    let request = SendMessageRequest {
        text: "背景を青くして".to_string(),
        messages: vec![
            MessageInfo {
                id: 0,
                is_user: false,
                text: "こんにちは！".to_string(),
            },
        ],
    };

    match send_message(request).await {
        Ok(response) => {
            println!("成功: {}", response.message);
            if let Some(styles) = response.chat_container_styles {
                println!("コンテナスタイル: {}", styles);
            }
        }
        Err(e) => {
            eprintln!("エラー: {:?}", e);
        }
    }
}
```

**注意事項:**
- この関数は`#[server]`マクロによりサーバーサイドでのみ実行されます
- クライアント側から呼び出すと、自動的にHTTPリクエストに変換されます
- 環境変数`GEMINI_API_KEY`が設定されている必要があります

---

## APIクライアント

### `send_message_to_api`

クライアント側からサーバーAPIを呼び出し、応答を処理する関数です。

**定義場所:** `app/src/api_client.rs`

**シグネチャ:**
```rust
pub fn send_message_to_api(params: ApiCallParams)
```

**パラメータ:**
- `params: ApiCallParams` - API呼び出しのパラメータ

**動作:**
1. ローディング状態を`true`に設定
2. 現在のメッセージ状態を`MessageInfo`のベクターに変換
3. `SendMessageRequest`を作成し、`send_message`サーバー関数を呼び出し
4. レスポンスを処理:
   - 新しい要素があれば、指定されたメッセージIDの後に挿入
   - スタイル更新があれば、各メッセージ要素のスタイルを更新
   - チャットコンテナのスタイルがあれば、適用
   - AIの応答メッセージをメッセージリストに追加
5. 一般的なスタイル変更要求の場合、新しいメッセージ（AIの返信）にもスタイルを適用
6. ローディング状態を`false`に戻す

**使用例:**
```rust
use app::api_client::{send_message_to_api, ApiCallParams};
use app::pages::chat_page::Message;
use common::DynamicElementData;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
fn MyComponent() -> impl IntoView {
    let (messages, set_messages) = create_signal(vec![Message {
        id: 0,
        text: "こんにちは！".to_string(),
        is_user: false,
    }]);
    let (is_loading, set_is_loading) = create_signal(false);
    let (chat_container_styles, set_chat_container_styles) = create_signal(String::new());
    let (dynamic_elements, set_dynamic_elements) = create_signal(HashMap::new());
    let (element_styles, set_element_styles) = create_signal(HashMap::new());

    let send_message = move |_| {
        send_message_to_api(ApiCallParams {
            user_message: "背景を青くして".to_string(),
            anchor_message_id: 0,
            current_messages: messages.get(),
            set_is_loading,
            set_messages,
            set_chat_container_styles,
            set_dynamic_elements,
            set_element_styles,
        });
    };

    view! {
        <button on:click=send_message>"送信"</button>
    }
}
```

---

### `ApiCallParams`

API呼び出しのパラメータを表す構造体です。

**定義場所:** `app/src/api_client.rs`

```rust
pub struct ApiCallParams {
    pub user_message: String,
    pub anchor_message_id: usize,
    pub current_messages: Vec<Message>,
    pub set_is_loading: WriteSignal<bool>,
    pub set_messages: WriteSignal<Vec<Message>>,
    pub set_chat_container_styles: WriteSignal<String>,
    pub set_dynamic_elements: WriteSignal<HashMap<usize, Vec<DynamicElementData>>>,
    pub set_element_styles: WriteSignal<HashMap<usize, String>>,
}
```

**フィールド:**
- `user_message: String` - ユーザーが送信するメッセージテキスト
- `anchor_message_id: usize` - 新しい要素を挿入する基準となるメッセージID
- `current_messages: Vec<Message>` - 現在のメッセージリスト
- `set_is_loading: WriteSignal<bool>` - ローディング状態を設定するシグナル
- `set_messages: WriteSignal<Vec<Message>>` - メッセージリストを更新するシグナル
- `set_chat_container_styles: WriteSignal<String>` - チャットコンテナのスタイルを設定するシグナル
- `set_dynamic_elements: WriteSignal<HashMap<usize, Vec<DynamicElementData>>>` - 動的要素を管理するシグナル
- `set_element_styles: WriteSignal<HashMap<usize, String>>` - 各メッセージ要素のスタイルを管理するシグナル

---

## CSSサニタイザー

### `CssSanitizer`

CSS文字列をサニタイズしてセキュリティリスクを軽減する構造体です。

**定義場所:** `app/src/css_sanitizer.rs`

**シグネチャ:**
```rust
pub struct CssSanitizer;
```

**メソッド:**

#### `new()`

新しい`CssSanitizer`インスタンスを作成します。

**シグネチャ:**
```rust
pub fn new() -> Self
```

**戻り値:**
- `Self` - 新しい`CssSanitizer`インスタンス

**使用例:**
```rust
use app::css_sanitizer::CssSanitizer;

let sanitizer = CssSanitizer::new();
```

#### `sanitize_css_string()`

CSS文字列をサニタイズし、危険なパターンを除去します。

**シグネチャ:**
```rust
pub fn sanitize_css_string(&self, css_string: &str) -> String
```

**パラメータ:**
- `css_string: &str` - サニタイズするCSS文字列

**戻り値:**
- `String` - サニタイズされたCSS文字列

**ブロックされる危険なパターン:**
- `javascript:`
- `vbscript:`
- `expression(`
- `eval(`
- `alert(`
- `document.`
- `window.`
- `location.`
- `history.`
- `navigator.`
- `<script`
- `</script`
- `onload=`
- `onerror=`
- `onclick=`
- `onmouseover=`
- `url(javascript:)`
- `url(vbscript:)`
- `url(data:text/html`
- `behavior:`
- `binding:`
- `-moz-binding:`
- `-webkit-binding:`

**処理内容:**
1. 危険なパターンを除去
2. 前後の空白をトリム
3. 最後のセミコロンを除去（統一性のため）
4. 空の場合は空文字列を返す

**使用例:**
```rust
use app::css_sanitizer::CssSanitizer;

let sanitizer = CssSanitizer::new();

// 安全なCSSプロパティはそのまま通る
let safe = sanitizer.sanitize_css_string("background-color: #ff0000; color: white;");
assert_eq!(safe, "background-color: #ff0000; color: white");

// 危険なパターンは除去される
let dangerous = sanitizer.sanitize_css_string("background-color: red; javascript: alert('xss');");
assert!(dangerous.contains("background-color: red"));
assert!(!dangerous.contains("javascript:"));
```

**テスト:**
```rust
#[test]
fn test_sanitize_safe_properties() {
    let sanitizer = CssSanitizer::new();
    
    assert_eq!(
        sanitizer.sanitize_css_string("background-color: #ff0000; color: white;"),
        "background-color: #ff0000; color: white"
    );
}

#[test]
fn test_block_dangerous_properties() {
    let sanitizer = CssSanitizer::new();
    
    let result = sanitizer.sanitize_css_string("background-color: red; javascript: alert('xss');");
    assert!(result.contains("background-color: red"));
    assert!(!result.contains("javascript:"));
}
```

---

## UIコンポーネント

### `shell`

HTMLシェルを生成する関数です。サーバーサイドレンダリング（SSR）で使用されます。

**定義場所:** `app/src/lib.rs`

**シグネチャ:**
```rust
pub fn shell(options: LeptosOptions) -> impl IntoView
```

**パラメータ:**
- `options: LeptosOptions` - Leptosの設定オプション

**戻り値:**
- `impl IntoView` - HTMLビュー

**生成されるHTML構造:**
```html
<!DOCTYPE html>
<html lang="ja">
    <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no, viewport-fit=cover"/>
        <AutoReload options=options.clone()/>
        <HydrationScripts options/>
        <MetaTags/>
    </head>
    <body>
        <App/>
    </body>
</html>
```

**使用例:**
```rust
use app::shell;
use leptos::prelude::*;

let app = shell(leptos_options);
```

---

### `App`

メインアプリケーションコンポーネントです。ルーティングとメタコンテキストを提供します。

**定義場所:** `app/src/lib.rs`

**シグネチャ:**
```rust
#[component]
pub fn App() -> impl IntoView
```

**機能:**
- メタコンテキストを提供（`provide_meta_context()`）
- スタイルシートを読み込み（`/pkg/self-changer.css`）
- ドキュメントタイトルを設定（`"Self Changer"`）
- ルーターを設定し、`ChatPage`コンポーネントをルートパス（`"/"`）にマッピング
- フォールバックページを設定（`"ページが見つかりません"`）

**レンダリングされる構造:**
```html
<Router>
    <main class="w-full h-screen flex flex-col items-center justify-center bg-gray-200 py-2">
        <Routes fallback=|| "ページが見つかりません".into_view()>
            <Route path=StaticSegment("") view=ChatPage/>
        </Routes>
    </main>
</Router>
```

**使用例:**
```rust
use app::App;

#[component]
fn MyEntryPoint() -> impl IntoView {
    view! {
        <App/>
    }
}
```

---

### `ChatPage`

チャットUIのホームページをレンダリングするコンポーネントです。

**定義場所:** `app/src/pages/chat_page.rs`

**シグネチャ:**
```rust
#[component]
pub fn ChatPage() -> impl IntoView
```

**機能:**
- メッセージリストの状態管理
- メッセージ入力フォームの状態管理
- API呼び出しの状態管理（ローディング状態）
- チャットコンテナ全体のスタイル管理
- 動的要素の管理
- 各メッセージ要素のスタイル管理

**状態管理:**
- `messages: Signal<Vec<Message>>` - メッセージリスト
- `new_message_text: Signal<String>` - 入力中のメッセージテキスト
- `is_loading: Signal<bool>` - API呼び出し中のローディング状態
- `chat_container_styles: Signal<String>` - チャットコンテナのCSSプロパティ
- `dynamic_elements: Signal<HashMap<usize, Vec<DynamicElementData>>>` - 動的要素のマップ
- `element_styles: Signal<HashMap<usize, String>>` - 各メッセージ要素のスタイルマップ

**イベントハンドラ:**
- `on_submit: SubmitEvent` - フォーム送信時の処理
  - デフォルトのフォーム動作を防止
  - メッセージをメッセージリストに追加
  - `send_message_to_api`を呼び出してAPI処理を実行
  - 入力フィールドをクリア

- `on_refresh: ClickEvent` - リフレッシュボタンクリック時の処理
  - メッセージリストを初期状態に戻す
  - 動的要素をクリア
  - スタイルマップを初期状態に戻す
  - コンテナのスタイルを初期状態に戻す

**レンダリングされる構造:**
```html
<div class="main-container">
    <!-- ローディングオーバーレイ -->
    <Show when=move || is_loading.get()>
        <div class="loading-overlay">
            <div class="loading-text">読み込み中...</div>
        </div>
    </Show>
    
    <!-- リフレッシュボタン -->
    <button on:click=on_refresh class="refresh-button">
        <svg>...</svg>
    </button>
    
    <!-- チャットコンテナ -->
    <div class="chat-container" style=move || chat_container_styles.get()>
        <!-- メッセージ履歴表示エリア -->
        <div class="messages-area">
            <For each=move || messages.get() key=|msg| msg.id>
                <!-- メッセージアイテム -->
                <div class="message-item {justify-end|justify-start}">
                    <!-- アイコン（AIメッセージのみ） -->
                    {(!msg.is_user).then(|| view! {
                        <div class="message-icon">🤖</div>
                    })}
                    
                    <!-- メッセージの吹き出し -->
                    <div style=msg_styles>
                        <p class="message-text">{msg.text}</p>
                    </div>
                </div>
                
                <!-- 動的要素 -->
                <For each=move || dynamic_elements.get().get(&msg.id)>
                    <!-- 動的要素のレンダリング -->
                </For>
            </For>
        </div>
        
        <!-- 入力フォーム -->
        <form on:submit=on_submit class="input-form">
            <input
                type="text"
                prop:value=new_message_text
                on:input=move |ev| set_new_message_text(event_target_value(&ev))
                placeholder="メッセージを入力..."
                class="input-field"
            />
            <button type="submit" class="send-button">
                <svg>...</svg>
            </button>
        </form>
    </div>
</div>
```

**サポートされている動的要素タグ:**
- `button`: ボタン要素
- `img`: 画像要素（`attributes`に`src`と`alt`を指定）
- `a`: リンク要素（`attributes`に`href`と`target`を指定）
- `p`: 段落要素
- `div`: 汎用コンテナ要素
- `span`: インライン要素
- `h1`, `h2`, `h3`, `h4`, `h5`, `h6`: 見出し要素
- `hr`: 区切り線要素
- `br`: 改行要素
- `input`: 入力要素（`attributes`に`type`、`placeholder`、`value`を指定）

**使用例:**
```rust
use app::pages::chat_page::ChatPage;

#[component]
fn MyApp() -> impl IntoView {
    view! {
        <ChatPage/>
    }
}
```

---

### `Message`

メッセージのデータを保持する構造体です。

**定義場所:** `app/src/pages/chat_page.rs`

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub id: usize,
    pub text: String,
    pub is_user: bool,
}
```

**フィールド:**
- `id: usize` - メッセージの一意なID（0から始まる連番）
- `text: String` - メッセージのテキスト内容
- `is_user: bool` - ユーザーメッセージの場合は`true`、AIメッセージの場合は`false`

**使用例:**
```rust
use app::pages::chat_page::Message;

let message = Message {
    id: 0,
    text: "こんにちは！".to_string(),
    is_user: false,
};
```

---

## サーバーエントリーポイント

### `main`

サーバーアプリケーションのエントリーポイント関数です。

**定義場所:** `server/src/main.rs`

**シグネチャ:**
```rust
#[tokio::main]
async fn main()
```

**動作:**
1. `.env`ファイルから環境変数を読み込む（`dotenv()`）
2. Leptosの設定を取得（`get_configuration(None)`）
3. サーバーアドレスを取得（`conf.leptos_options.site_addr`）
4. ルートリストを生成（`generate_route_list(App)`）
5. Axumルーターを設定:
   - Leptosルートを設定（`leptos_routes()`）
   - フォールバックハンドラを設定（`file_and_error_handler()`）
   - Leptosオプションを状態として設定
6. TCPリスナーをバインドしてサーバーを起動

**使用例:**
```bash
# サーバーを起動
cargo run --bin server

# または、環境変数を設定して起動
GEMINI_API_KEY=your_key cargo run --bin server
```

**環境変数:**
- `GEMINI_API_KEY`: Google Gemini APIキー（必須）

**設定:**
- サーバーアドレスは`leptos.toml`の`site-addr`で設定（デフォルト: `127.0.0.1:3000`）

---

## フロントエンドエントリーポイント

### `hydrate`

WASMハイドレーション関数です。クライアント側でサーバーサイドレンダリングされたHTMLをハイドレートします。

**定義場所:** `frontend/src/lib.rs`

**シグネチャ:**
```rust
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate()
```

**動作:**
1. `console_log`を使用してログを初期化（`log::Level::Debug`）
2. パニックフックを設定（`console_error_panic_hook::set_once()`）
3. Leptosのハイドレーションを実行（`leptos::mount::hydrate_body(App)`）

**使用例:**
```html
<!-- HTMLファイル内で -->
<script type="module">
    import init, { hydrate } from '/pkg/self-changer.js';
    init().then(() => {
        hydrate();
    });
</script>
```

**注意事項:**
- この関数はWASMモジュールとしてエクスポートされます
- サーバーサイドレンダリングされたHTMLが存在する必要があります
- `App`コンポーネントがサーバー側でレンダリングされている必要があります

---

## 使用例

### 基本的な使い方

```rust
use app::pages::chat_page::ChatPage;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <ChatPage/>
    }
}
```

### API呼び出しの例

```rust
use app::api_client::{send_message_to_api, ApiCallParams};
use app::pages::chat_page::Message;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
fn MyComponent() -> impl IntoView {
    let (messages, set_messages) = create_signal(vec![Message {
        id: 0,
        text: "こんにちは！".to_string(),
        is_user: false,
    }]);
    let (is_loading, set_is_loading) = create_signal(false);
    let (chat_container_styles, set_chat_container_styles) = create_signal(String::new());
    let (dynamic_elements, set_dynamic_elements) = create_signal(HashMap::new());
    let (element_styles, set_element_styles) = create_signal(HashMap::new());

    let send_message = move |_| {
        send_message_to_api(ApiCallParams {
            user_message: "背景を青くして".to_string(),
            anchor_message_id: messages.get().len(),
            current_messages: messages.get(),
            set_is_loading,
            set_messages,
            set_chat_container_styles,
            set_dynamic_elements,
            set_element_styles,
        });
    };

    view! {
        <button on:click=send_message>"送信"</button>
    }
}
```

### CSSサニタイザーの使用例

```rust
use app::css_sanitizer::CssSanitizer;

fn sanitize_user_input(user_css: &str) -> String {
    let sanitizer = CssSanitizer::new();
    sanitizer.sanitize_css_string(user_css)
}
```

---

## エラーハンドリング

### `ServerFnError`

サーバー関数で発生するエラーを表す型です。

**主なエラーケース:**
1. **APIキーが見つからない場合:**
   ```rust
   ServerFnError::new("APIキーが見つかりません: {error}")
   ```

2. **クライアント初期化エラー:**
   ```rust
   ServerFnError::new("クライアント初期化エラー: {error}")
   ```

3. **API呼び出しエラー:**
   ```rust
   ServerFnError::new("API呼び出しエラー: {error}")
   ```

4. **JSON解析エラー:**
   - フォールバックとして`success: false`のレスポンスを返す

**エラーハンドリングの例:**
```rust
match send_message(request).await {
    Ok(response) => {
        if response.success {
            println!("成功: {}", response.message);
        } else {
            eprintln!("失敗: {}", response.message);
        }
    }
    Err(e) => {
        eprintln!("エラー: {:?}", e);
    }
}
```

---

## セキュリティ考慮事項

### CSSサニタイゼーション

すべてのCSSプロパティは`CssSanitizer`を通してサニタイズされます。以下の危険なパターンがブロックされます:

- JavaScript実行（`javascript:`, `eval()`, `expression()`など）
- DOM操作（`document.`, `window.`など）
- イベントハンドラ（`onclick=`, `onerror=`など）
- スクリプトインジェクション（`<script>`タグなど）

### 推奨事項

1. **APIキーの保護**: `GEMINI_API_KEY`は環境変数として管理し、リポジトリにコミットしないでください
2. **入力検証**: ユーザー入力は常に検証してください
3. **エラーハンドリング**: すべてのAPI呼び出しでエラーハンドリングを実装してください
4. **ログ**: 本番環境では適切なログレベルを設定してください

---

## 追加リソース

- [Leptos公式ドキュメント](https://docs.rs/leptos/)
- [Axum公式ドキュメント](https://docs.rs/axum/)
- [Google Gemini API ドキュメント](https://ai.google.dev/docs)
- [プロジェクトREADME](README.md)

---

## 更新履歴

- 2024年: 初版作成
