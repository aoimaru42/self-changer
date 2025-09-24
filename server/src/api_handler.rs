use common::*;
use leptos::prelude::ServerFnError;
use google_ai_rs::client::Client;
use leptos::serde_json::from_str;

#[allow(dead_code)]
pub async fn send_message(req: SendMessageRequest) -> Result<SendMessageResponse, ServerFnError> {
    log::info!("Received message: {}", req.text);

    // 1. 環境変数から API キーを取得
    let api_key = std::env::var("GEMINI_API_KEY")
        .map_err(|e| ServerFnError::new(format!("APIキーが見つかりません: {}", e)))?;

    // 2. モデルを初期化
    let client = Client::new(api_key).await
        .map_err(|e| ServerFnError::new(format!("クライアント初期化エラー: {}", e)))?;

    let model = client.generative_model("gemini-2.0-flash");
    
    let mut chat = model.start_chat();
    // 3. プロンプトを構築
    // JSONの出力形式をAIに厳密に指示するプロンプトを作成します。
    // `serde_json` の `to_string_pretty` を使うと、例を簡単に生成できます。
    let prompt = format!(
        r#"あなたはUI変更のためのJSONデータを生成するアシスタントです。
        
        ### ルール
        1. ユーザーのリクエストに基づいてUIを動的に変更するJSONを生成してください。
        2. 出力はJSON形式のみとし、余計な説明文やマークダウンは含めないでください。
        
        ### JSONスキーマ
        ```json
        {{
            "success": bool,
            "chat_container_class": Option<String>,
            "change_style_elements": Option<Vec<StyleUpdate>>,
            "new_elements": Option<Vec<DynamicElementData>>
        }}
        ```

        ### 例
        ユーザー: 「全体背景を青くして」
        JSON: {{"success": true, "chat_container_class": "bg-blue-500", "change_style_elements": [], "new_elements": []}}

        ユーザー: 「チャットの２つ目の要素の背景を青くして」
        JSON: {{"success": true, "chat_container_class": null, "change_style_elements": [{{"id": 2, "classes": "bg-blue-500"}}], "new_elements": []}}

        ユーザー: 「ボタンを追加して」
        JSON: {{"success": true, "chat_container_class": null, "change_style_elements": [], "new_elements": [{{"id": 1, "tag": "button", "text": "新しいボタン", "classes": "bg-green-500", "attributes": {{}}}}]}}

        ### ユーザーリクエスト
        {}

        ### JSON出力
        "#,
        req.text
    );

    // let response_chat_container_class = Some("bg-gray-500".to_string());

    // let response_style = Some(vec![
    //     StyleUpdate {
    //         id: 1,
    //         classes: "bg-red-500".to_string()
    //     }
    // ]);

    // let response_add = Some(vec![
    //     DynamicElementData {
    //         id: 1,
    //         tag: "button".to_string(),
    //         text: Some("新しいボタン".to_string()),
    //         classes: Some("bg-green-500 text-white p-3 rounded-md".to_string()),
    //         attributes: None,
    //     }
    // ]);

    // 4. API を呼び出す
    let response = chat.send_message(prompt)
        .await
        .map_err(|e| ServerFnError::new(format!("API呼び出しエラー: {}", e)))?;

    println!("{:?}", response);

    // 5. 応答を処理し、JSON をパース
    let json_str = response
        .candidates
        .first()
        .and_then(|c| c.content.as_ref()) 
        .and_then(|c| c.parts.first())
        .map(|p|  p.to_text())
        // .and_then(|p| match p {
            // Part::Text(t) => Some(t),
            // _ => None,
        // })
        // .and_then(|p| p.text())
        .ok_or(ServerFnError::new("AIからの応答がテキストではありませんでした".to_string()))?;

    let response_data: SendMessageResponse = from_str(json_str)
        .map_err(|e| ServerFnError::new(format!("JSONパースエラー: {}", e)))?;

    Ok(response_data)
}