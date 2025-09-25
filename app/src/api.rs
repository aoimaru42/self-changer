use common::*;
#[cfg(feature = "ssr")]
use google_ai_rs::client::Client;
use leptos::prelude::ServerFnError;
#[cfg(feature = "ssr")]
use leptos::serde_json::Value;
use leptos::server;

#[server]
pub async fn send_message(_req: SendMessageRequest) -> Result<SendMessageResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // 1. APIキー取得
        let api_key = std::env::var("GEMINI_API_KEY")
            .map_err(|e| ServerFnError::new(format!("APIキーが見つかりません: {}", e)))?;

        // 2. クライアント初期化
        let client = Client::new(api_key)
            .await
            .map_err(|e| ServerFnError::new(format!("クライアント初期化エラー: {}", e)))?;

        let model = client.generative_model("gemini-2.0-flash");
        let mut chat = model.start_chat();

        // 3. プロンプト作成（拡張版）
        let prompt_template = r#"あなたはUI変更のためのJSONデータを生成するアシスタントです。

            ルール:
            - 出力は純粋なJSONのみ
            - CSSプロパティのみを使用（background-color, color, font-size, font-family, font-weight, border, padding, margin等）
            - 危険なプロパティ（javascript:, expression()等）は禁止
            - スタイル変更は永続的に適用される
            - 特定要素指定時は他の要素のスタイルを保持する
            - 「文字の色」「文字サイズ」「文字の太さ」等の指示は、既存の全てのメッセージ要素（ID: 0, 1, 2...）に適用する
            - 要素のIDは0から始まり、現在のメッセージ数に応じて増加する
            - メッセージ要素のスタイルは、親のdiv要素に適用して子要素のpタグ（message-textクラス）に継承させる
            - フォント関連のスタイル（color, font-size, font-weight等）は親要素に適用することで子要素に継承される

            現在のメッセージ状態:
            {MESSAGE_CONTEXT}

            対応可能な操作:

            1. スタイル変更:
            - 全体背景を青くして: {"success": true, "message": "背景を青に変更しました", "chat_container_styles": "background-color: #3b82f6;", "change_style_elements": [], "new_elements": []}
            - 2番目の要素を青くして: {"success": true, "message": "2番目の吹き出しを青にしました", "chat_container_styles": "", "change_style_elements": [{"id": 2, "styles": "background-color: #3b82f6; color: white;"}], "new_elements": []}
            - 文字を太字にして: {"success": true, "message": "文字を太字にしました", "chat_container_styles": "", "change_style_elements": [{"id": 0, "styles": "font-weight: bold;"}, {"id": 1, "styles": "font-weight: bold;"}], "new_elements": []}
            - 文字の色を白にして: {"success": true, "message": "文字の色を白に変更しました", "chat_container_styles": "", "change_style_elements": [{"id": 0, "styles": "color: white;"}, {"id": 1, "styles": "color: white;"}], "new_elements": []}
            - 文字サイズを大きくして: {"success": true, "message": "文字サイズを大きくしました", "chat_container_styles": "", "change_style_elements": [{"id": 0, "styles": "font-size: 18px;"}, {"id": 1, "styles": "font-size: 18px;"}], "new_elements": []}

            2. 新しい要素の追加:
            - ボタンを追加して: {"success": true, "message": "ボタンを追加しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 0, "tag": "button", "text": "クリックしてください", "styles": "background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer;", "attributes": null}]}
            - 画像を追加して: {"success": true, "message": "画像を追加しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 0, "tag": "img", "text": null, "styles": "max-width: 100%; height: auto; border-radius: 8px; margin: 10px 0; display: block;", "attributes": {"src": "https://picsum.photos/300/200", "alt": "サンプル画像"}}]}
            - 猫の画像を表示して: {"success": true, "message": "猫の画像を表示しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 0, "tag": "img", "text": null, "styles": "max-width: 300px; height: 200px; border-radius: 10px; margin: 15px auto; display: block; box-shadow: 0 4px 8px rgba(0,0,0,0.1);", "attributes": {"src": "https://cataas.com/cat", "alt": "可愛い猫の画像"}}]}
            - リンクを追加して: {"success": true, "message": "リンクを追加しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 0, "tag": "a", "text": "こちらをクリック", "styles": "color: #007bff; text-decoration: underline; font-weight: bold;", "attributes": {"href": "https://example.com"}}]}
            - リンクボタンを作って: {"success": true, "message": "リンクボタンを作成しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 0, "tag": "a", "text": "Googleへ移動", "styles": "display: inline-block; background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer; text-decoration: none; font-weight: bold;", "attributes": {"href": "https://google.com", "target": "_blank"}}]}
            - google.comに飛ぶボタンを作って: {"success": true, "message": "Googleに飛ぶボタンを作成しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 0, "tag": "a", "text": "Googleへ", "styles": "display: inline-block; background-color: #4285f4; color: white; padding: 12px 24px; border: none; border-radius: 6px; cursor: pointer; text-decoration: none; font-weight: bold; transition: background-color 0.3s;", "attributes": {"href": "https://google.com", "target": "_blank"}}]}
            - テキストを追加して: {"success": true, "message": "テキストを追加しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 0, "tag": "p", "text": "追加されたテキストです", "styles": "color: #333; font-size: 16px; margin: 10px 0;", "attributes": null}]}
            - 区切り線を追加して: {"success": true, "message": "区切り線を追加しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 0, "tag": "hr", "text": null, "styles": "border: none; height: 2px; background-color: #ddd; margin: 20px 0;", "attributes": null}]}

            3. 複合操作:
            - 背景を変えてボタンも追加して: {"success": true, "message": "背景を変更し、ボタンも追加しました", "chat_container_styles": "background-color: #f8f9fa;", "change_style_elements": [], "new_elements": [{"id": 0, "tag": "button", "text": "新しいボタン", "styles": "background-color: #28a745; color: white; padding: 12px 24px; border: none; border-radius: 6px; cursor: pointer; margin: 10px 0;", "attributes": null}]}

            利用可能なHTMLタグ:
            - button: ボタン要素（textフィールドにボタンテキスト、attributesはnull）
            - img: 画像要素（textはnull、attributesにsrcとaltを指定、stylesにdisplay: blockを推奨）
            - a: リンク要素（textフィールドにリンクテキスト、attributesにhrefとtargetを指定）
            - p: 段落要素（textフィールドにテキスト、attributesはnull）
            - div: 汎用コンテナ要素（textフィールドにテキスト、attributesはnull）
            - span: インライン要素（textフィールドにテキスト、attributesはnull）
            - h1, h2, h3, h4, h5, h6: 見出し要素（textフィールドにテキスト、attributesはnull）
            - hr: 区切り線要素（textはnull、attributesはnull）
            - br: 改行要素（textはnull、attributesはnull）

            画像の重要なポイント:
            - 必ず<img>タグを使用
            - src属性に完全なURLを指定（https://から始まる）
            - alt属性で画像の説明を提供
            - stylesにdisplay: blockを追加して適切に表示
            - max-width: 100%でレスポンシブ対応
            - border-radiusで角を丸くする

            リンクボタンの重要なポイント:
            - 外部サイトへのリンクは必ず<a>タグを使用（<button>タグでは外部リンク不可）
            - href属性に完全なURLを指定（https://から始まる）
            - target="_blank"を指定して新しいタブで開く
            - text-decoration: noneでアンダーラインを消す
            - display: inline-blockでブロック要素として表示

            重要: 全ての要素には必ずidフィールドを含めること（通常は0から開始）

            ユーザーリクエスト: {USER_REQ}

            JSON出力:"#;

        let prompt = prompt_template
            .replace("{USER_REQ}", &_req.text)
            .replace("{MESSAGE_CONTEXT}", &format!(
                "現在のメッセージ一覧（総数: {}）:\n{}\n\n注意: ユーザーが新しいメッセージを送信した後、AIの返信メッセージのIDは {} になります。",
                _req.messages.len(),
                _req.messages.iter()
                    .map(|msg| format!("ID: {}, is_user: {}, text: \"{}\"", msg.id, msg.is_user, msg.text))
                    .collect::<Vec<_>>()
                    .join("\n"),
                _req.messages.len()
            ));

        // 4. モデル呼び出し
        let response = chat
            .send_message(prompt)
            .await
            .map_err(|e| ServerFnError::new(format!("API呼び出しエラー: {}", e)))?;

        // 5. JSON抽出
        let raw_text = response
            .candidates
            .first()
            .and_then(|c| c.content.as_ref())
            .and_then(|c| c.parts.first())
            .map(|p| p.to_text())
            .ok_or(ServerFnError::new(
                "AIからの応答がテキストではありませんでした".to_string(),
            ))?;

        // コードフェンスや "JSON:" 接頭辞を除去
        fn normalize_ai_output(text: &str) -> String {
            let mut s = text.trim().to_string();
            if let Some(pos) = s.to_lowercase().find("json:") {
                s = s[(pos + 5)..].trim().to_string();
            }
            if s.starts_with("```") {
                let mut t = s.trim_start_matches('`').to_string();
                if let Some(idx) = t.find('\n') {
                    t = t[idx + 1..].to_string();
                }
                if let Some(end) = t.rfind("```") {
                    t = t[..end].trim().to_string();
                }
                s = t;
            }
            s
        }

        // 最初の完全なJSONオブジェクトを抽出
        fn extract_first_json_object(text: &str) -> Option<String> {
            let bytes = text.as_bytes();
            let mut start = None;
            let mut depth: i32 = 0;
            for (i, &b) in bytes.iter().enumerate() {
                if b == b'{' {
                    if start.is_none() {
                        start = Some(i);
                    }
                    depth += 1;
                } else if b == b'}' {
                    if depth > 0 {
                        depth -= 1;
                    }
                    if depth == 0 {
                        if let Some(s) = start {
                            return Some(text[s..=i].to_string());
                        }
                    }
                }
            }
            None
        }

        let normalized = normalize_ai_output(raw_text.trim());
        let candidate_json =
            extract_first_json_object(&normalized).unwrap_or_else(|| normalized.clone());

        // 6. CSSサニタイザーを初期化
        use crate::css_sanitizer::CssSanitizer;
        let sanitizer = CssSanitizer::new();

        // 7. JSON解析とサニタイズ
        let v: Value = match leptos::serde_json::from_str::<Value>(&candidate_json) {
            Ok(val) => val,
            Err(e) => {
                log::warn!(
                    "JSON parse failed. Fallback applied. error={}, content={}",
                    e,
                    candidate_json
                );
                return Ok(SendMessageResponse {
                    success: false,
                    message: "AIの出力がJSONとして不正だったため、UIは変更していません。もう一度具体的に指示してください。".to_string(),
                    chat_container_styles: None,
                    change_style_elements: None,
                    new_elements: None,
                });
            }
        };

        // 基本的な型チェック
        if v.get("success").and_then(|x| x.as_bool()).is_none() {
            return Err(ServerFnError::new(
                "JSONフィールド 'success' が bool ではありません",
            ));
        }
        if v.get("message").and_then(|x| x.as_str()).is_none() {
            return Err(ServerFnError::new(
                "JSONフィールド 'message' が string ではありません",
            ));
        }

        // 8. データ抽出とサニタイズ
        let success = v.get("success").and_then(|x| x.as_bool()).unwrap_or(false);
        let message = v
            .get("message")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string();

        let styles_str = v
            .get("chat_container_styles")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let chat_container_styles = if styles_str.is_empty() {
            None
        } else {
            let sanitized = sanitizer.sanitize_css_string(&styles_str);
            if sanitized.is_empty() {
                None
            } else {
                Some(sanitized)
            }
        };

        let style_val = v
            .get("change_style_elements")
            .cloned()
            .unwrap_or_else(|| Value::Array(vec![]));
        let mut styles: Vec<StyleUpdate> =
            leptos::serde_json::from_value(style_val).unwrap_or_default();

        // スタイル更新をサニタイズ
        styles.retain_mut(|style| {
            let sanitized = sanitizer.sanitize_css_string(&style.styles);
            if !sanitized.is_empty() {
                style.styles = sanitized;
                true
            } else {
                false
            }
        });

        let change_style_elements = if styles.is_empty() {
            None
        } else {
            Some(styles)
        };

        let new_val = v
            .get("new_elements")
            .cloned()
            .unwrap_or_else(|| Value::Array(vec![]));
        let mut news: Vec<DynamicElementData> =
            leptos::serde_json::from_value(new_val).unwrap_or_default();

        // 新しい要素のスタイルをサニタイズ
        for element in &mut news {
            if let Some(ref mut styles) = element.styles {
                let sanitized = sanitizer.sanitize_css_string(styles);
                if sanitized.is_empty() {
                    element.styles = None;
                } else {
                    *styles = sanitized;
                }
            }
        }

        let new_elements = if news.is_empty() { None } else { Some(news) };

        Ok(SendMessageResponse {
            success,
            message,
            chat_container_styles,
            change_style_elements,
            new_elements,
        })
    }

    #[cfg(not(feature = "ssr"))]
    {
        // クライアント側では、この関数は自動的にHTTPリクエストに変換されます
        unreachable!("Server function should not be called directly on client side")
    }
}
