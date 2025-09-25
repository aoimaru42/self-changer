use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// メッセージ送信APIへのリクエストボディ
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub text: String,
    pub messages: Vec<MessageInfo>, // 現在のメッセージ履歴
}

// メッセージ情報（AIにコンテキストを提供するため）
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageInfo {
    pub id: usize,
    pub is_user: bool,
    pub text: String,
}

// メッセージ送信APIからのレスポンス
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub success: bool,
    pub message: String,
    pub chat_container_styles: Option<String>, // CSSプロパティ文字列
    pub change_style_elements: Option<Vec<StyleUpdate>>,
    pub new_elements: Option<Vec<DynamicElementData>>,
}

// 動的に生成する要素のデータを表現する汎用的な構造体
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicElementData {
    pub id: usize,
    pub tag: String,
    pub text: Option<String>,
    pub styles: Option<String>, // CSSプロパティ文字列（classesから変更）
    pub attributes: Option<HashMap<String, String>>,
}

// 既存の要素のスタイルを変更するための構造体
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StyleUpdate {
    pub id: usize,
    pub styles: String, // CSSプロパティ文字列（classesから変更）
}
