use leptos::server;
use common::*;
use leptos::prelude::ServerFnError;
#[cfg(feature = "ssr")]
use google_ai_rs::client::Client;
#[cfg(feature = "ssr")]
use leptos::serde_json::Value;

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

        // 3. プロンプト作成（CSSプロパティ形式）
        let prompt_template = r#"あなたはUI変更のためのJSONデータを生成するアシスタントです。
            
            ルール（厳守）
            - 出力は純粋なJSONのみ。説明・マークダウン・コードフェンスを含めない。
            - フィールドは必ずすべて含める。null は使わない。
            - chat_container_styles: CSSプロパティ文字列。未変更は空文字 ""。
            - change_style_elements, new_elements: 必ず配列。未変更は []。
            - CSSプロパティのみを使用すること（background-color, color, font-size, margin, padding, border-radius等）。Tailwind CSSクラスは禁止。
            - CSSプロパティは標準的な形式で記述（例: "background-color: #ff0000; color: white; font-size: 16px;"）
            - 色の指定: #ff0000, rgb(255,0,0), rgba(255,0,0,0.5), hsl(0,100%,50%) 等の標準形式
            - サイズ指定: px, em, rem, %, vh, vw 等の標準単位
            - 画像表示: imgタグを使用し、attributesにsrc（画像URL）とalt（代替テキスト）を指定
            - 危険なプロパティ（javascript:, expression(), url(javascript:)等）は絶対に使用しない
            - プレースホルダ（{answer} や {new_elements} 等）やテンプレ文字列は禁止。必ず具体的な値のみ。
            - UI変更に関係ないメッセージを受け付けた場合は、messageフィールドに UIの更新をしていない旨のメッセージを返す。
            - 型:
              success: bool
              message: string
              chat_container_styles: string
              change_style_elements: Array<{"id": number, "styles": string}>
              new_elements: Array<{"id": number, "tag": string, "text"?: string, "styles"?: string, "attributes"?: object}>
            
            現在のメッセージ状態:
            {MESSAGE_CONTEXT}
            
            例1: 全体背景を青くして
            {"success": true, "message": "背景を青に変更しました", "chat_container_styles": "background-color: #3b82f6;", "change_style_elements": [], "new_elements": []}
            例2: チャットの2つ目の要素の背景を青くして
            {"success": true, "message": "2番目の吹き出しを青にしました", "chat_container_styles": "", "change_style_elements": [{"id": 2, "styles": "background-color: #3b82f6; color: white;"}], "new_elements": []}
            例3: 最初のメッセージを太字にして
            {"success": true, "message": "最初のメッセージを太字にしました", "chat_container_styles": "", "change_style_elements": [{"id": 0, "styles": "font-weight: bold;"}], "new_elements": []}
            例4: 文字を大きくして
            {"success": true, "message": "文字サイズを大きくしました", "chat_container_styles": "", "change_style_elements": [{"id": 0, "styles": "font-size: 20px;"}], "new_elements": []}
            例5: 吹き出しを黄色にして（全ての吹き出しに適用）
            {"success": true, "message": "全ての吹き出しを黄色にしました", "chat_container_styles": "", "change_style_elements": [{"id": 0, "styles": "background-color: #ffff00;"}, {"id": 1, "styles": "background-color: #ffff00;"}, {"id": 2, "styles": "background-color: #ffff00;"}], "new_elements": []}
            例6: ユーザー側の吹き出しを黄色、AI側の吹き出しを紫にして
            {"success": true, "message": "ユーザー側の吹き出しを黄色、AI側の吹き出しを紫にしました", "chat_container_styles": "", "change_style_elements": [{"id": 0, "styles": "background-color: #ffff00;"}, {"id": 1, "styles": "background-color: #800080; color: white;"}, {"id": 2, "styles": "background-color: #ffff00;"}], "new_elements": []}
            例7: ボタンを追加して
            {"success": true, "message": "ボタンを追加しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 1, "tag": "button", "text": "新しいボタン", "styles": "background-color: #10b981; color: white; font-weight: bold; padding: 8px 16px; border-radius: 4px;", "attributes": {}}]}
            例8: 画像を表示して
            {"success": true, "message": "画像を表示しました", "chat_container_styles": "", "change_style_elements": [], "new_elements": [{"id": 1, "tag": "img", "text": null, "styles": "width: 256px; height: 192px; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);", "attributes": {"src": "https://picsum.photos/400/300", "alt": "サンプル画像"}}]}
            
            重要なルール:
            - 一般的なスタイル変更要求（「吹き出しを○○にして」「メッセージを○○にして」等）の場合、現在存在する全てのメッセージ要素（id: 0, 1, 2, ...）に同じスタイルを適用する
            - 特定要素指定（「最初の」「2番目の」等）の場合、指定された要素のみに適用する
            - ユーザー側指定（「ユーザー側」「ユーザーの吹き出し」「私の吹き出し」等）の場合、is_user=trueのメッセージ（通常は奇数ID: 1, 3, 5...）に適用する
            - AI側指定（「AI側」「AIの吹き出し」「ボットの吹き出し」等）の場合、is_user=falseのメッセージ（通常は偶数ID: 0, 2, 4...）に適用する
            - 新要素追加後はIDマッピングが変動する可能性があるため、現在のメッセージ構造を考慮して適切なIDを選択する
            - CSSプロパティの最後には必ずセミコロン（;）を付ける
            - JSON構文エラー防止のため、stylesフィールドの値は必ずダブルクォートで囲む
            
            ユーザーリクエスト
            {{USER_REQ}}
            
            JSON出力
            "#;

        let prompt = prompt_template
            .replace("{USER_REQ}", &_req.text)
            .replace("{MESSAGE_CONTEXT}", &format!(
                "現在のメッセージ一覧:\n{}",
                _req.messages.iter()
                    .map(|msg| format!("ID: {}, is_user: {}, text: \"{}\"", msg.id, msg.is_user, msg.text))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));

        // 4. モデル呼び出し
        let response = chat
            .send_message(prompt)
            .await
            .map_err(|e| ServerFnError::new(format!("API呼び出しエラー: {}", e)))?;

        println!("AIからの応答: {:?}", response);

        // 5. JSON抽出（正規化を行う）
        let raw_text = response
            .candidates
            .first()
            .and_then(|c| c.content.as_ref())
            .and_then(|c| c.parts.first())
            .map(|p| p.to_text())
            .ok_or(ServerFnError::new("AIからの応答がテキストではありませんでした".to_string()))?;

        // コードフェンスや "JSON:" 接頭辞を除去
        fn normalize_ai_output(text: &str) -> String {
            let mut s = text.trim().to_string();
            if let Some(pos) = s.to_lowercase().find("json:") {
                s = s[(pos + 5)..].trim().to_string();
            }
            if s.starts_with("```") {
                let mut t = s.trim_start_matches('`').to_string();
                if let Some(idx) = t.find('\n') { t = t[idx + 1..].to_string(); }
                if let Some(end) = t.rfind("```") { t = t[..end].trim().to_string(); }
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
                    if start.is_none() { start = Some(i); }
                    depth += 1;
                } else if b == b'}' {
                    if depth > 0 { depth -= 1; }
                    if depth == 0 { if let Some(s) = start { return Some(text[s..=i].to_string()); } }
                }
            }
            None
        }

        let normalized = normalize_ai_output(raw_text.trim());
        let candidate_json = extract_first_json_object(&normalized).unwrap_or_else(|| normalized.clone());

        // 6. CSSサニタイザーを初期化
        use crate::css_sanitizer::CssSanitizer;
        let sanitizer = CssSanitizer::new();

        // 7. 緩いスキーマ検証（失敗時はUI変更なしのフォールバックを返す）
        let v: Value = match leptos::serde_json::from_str::<Value>(&candidate_json) {
            Ok(val) => val,
            Err(e) => {
                log::warn!("JSON parse failed. Fallback applied. error={}, content={}", e, candidate_json);
                return Ok(SendMessageResponse {
                    success: false,
                    message: "AIの出力がJSONとして不正だったため、UIは変更していません。もう一度具体的に指示してください。".to_string(),
                    chat_container_styles: None,
                    change_style_elements: None,
                    new_elements: None,
                });
            }
        };

        if v.get("success").and_then(|x| x.as_bool()).is_none() {
            return Err(ServerFnError::new("JSONフィールド 'success' が bool ではありません"));
        }
        if v.get("message").and_then(|x| x.as_str()).is_none() {
            return Err(ServerFnError::new("JSONフィールド 'message' が string ではありません"));
        }
        // 型チェック（null を許容して後で正規化）
        if let Some(x) = v.get("chat_container_styles") { if !x.is_null() && !x.is_string() { return Err(ServerFnError::new("JSONフィールド 'chat_container_styles' は string か null")); } }
        if let Some(x) = v.get("change_style_elements") { if !x.is_null() && !x.is_array() { return Err(ServerFnError::new("JSONフィールド 'change_style_elements' は配列か null")); } }
        if let Some(x) = v.get("new_elements") { if !x.is_null() && !x.is_array() { return Err(ServerFnError::new("JSONフィールド 'new_elements' は配列か null")); } }

        // 8. 正規化して構造体を手動構築（CSSサニタイズ付き）
        let success = v.get("success").and_then(|x| x.as_bool()).unwrap_or(false);
        let message = v.get("message").and_then(|x| x.as_str()).unwrap_or("").to_string();

        let styles_str = v.get("chat_container_styles").and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
        let chat_container_styles = if styles_str.is_empty() { 
            None 
        } else { 
            let sanitized = sanitizer.sanitize_css_string(&styles_str);
            if sanitized.is_empty() { None } else { Some(sanitized) }
        };

        let style_val = v.get("change_style_elements").cloned().unwrap_or_else(|| Value::Array(vec![]));
        let mut styles: Vec<StyleUpdate> = leptos::serde_json::from_value(style_val).unwrap_or_default();
        
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
        
        let change_style_elements = if styles.is_empty() { None } else { Some(styles) };

        let new_val = v.get("new_elements").cloned().unwrap_or_else(|| Value::Array(vec![]));
        let mut news: Vec<DynamicElementData> = leptos::serde_json::from_value(new_val).unwrap_or_default();
        
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

        Ok(SendMessageResponse { success, message, chat_container_styles, change_style_elements, new_elements })
    }

    #[cfg(not(feature = "ssr"))]
    {
        // クライアント側では、この関数は自動的にHTTPリクエストに変換されます
        // この部分は実際には実行されません
        unreachable!("Server function should not be called directly on client side")
    }
}
