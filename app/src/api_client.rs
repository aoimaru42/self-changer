use leptos::prelude::*;
use leptos::prelude::WriteSignal;
use leptos::task::spawn_local;

use crate::api::send_message;
use crate::pages::chat_page::Message;
use std::collections::HashMap;
use common::*;

/// API呼び出しのパラメータ
#[derive(Clone)]
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

/// メッセージをサーバーに送信し、応答を処理します。
pub fn send_message_to_api(params: ApiCallParams) {
    // API呼び出し中はローディング状態をtrueに設定
    params.set_is_loading.set(true);

    // 非同期にAPIを呼び出す
    spawn_local(async move {
        // 現在のメッセージ状態を取得
        let message_context: Vec<common::MessageInfo> = params.current_messages.iter()
            .map(|msg| common::MessageInfo {
                id: msg.id,
                is_user: msg.is_user,
                text: msg.text.clone(),
            })
            .collect();
        
        let req = SendMessageRequest { 
            text: params.user_message.clone(),
            messages: message_context,
        };
        let api_response = send_message(req).await;
        // let api_response = "test".to_string();

        // let api_response = SendMessageResponse {
        //     success: true,
        //     response_text: "新しい要素を追加しました！".to_string(),
        //     change_style_elements: Some(vec![
        //         StyleUpdate {
        //             id: 1,
        //             classes: "bg-red-500".to_string()
        //         },
        //     ]),
        //     new_elements: Some(vec![
        //         DynamicElementData {
        //             id: 1,
        //             tag: "button".to_string(),
        //             text: Some("新しいボタン".to_string()),
        //             classes: Some("bg-green-500 text-white p-3 rounded-md".to_string()),
        //             attributes: None,
        //         },
        //         DynamicElementData {
        //             id: 2,
        //             tag: "p".to_string(),
        //             text: Some("これは動的に追加されたテキストです。".to_string()),
        //             classes: Some("text-lg font-semibold mt-4".to_string()),
        //             attributes: None,
        //         },
        //         DynamicElementData {
        //             id: 3,
        //             tag: "div".to_string(),
        //             text: None,
        //             classes: Some("absolute sakura-petal rounded-full w-2 h-2 bg-pink-300 opacity-75 animate-fall".to_string()),
        //             attributes: None,
        //         }
        //     ]),
        // };

        // スタイルコマンドが存在する場合、クラスを更新
        // if let Some(style_command) = api_response.style_command {
        //     set_chat_container_class.set(format!("{} {}", style_command, base_classes));
        // }
        match api_response {
            Ok(res) => {
                // 新しい要素を指定されたメッセージIDの後に挿入
                if let Some(elements) = res.new_elements {
                    params.set_dynamic_elements.update(|map| {
                        map.insert(params.anchor_message_id, elements);
                    });
                }
        
                // スタイル更新はCSSプロパティ形式で処理
                let element_updates_clone = res.change_style_elements.clone();
                if let Some(element_updates) = element_updates_clone {
                    params.set_element_styles.update(|styles| {
                        for update in element_updates {
                            styles
                                .entry(update.id)
                                .and_modify(|existing| {
                                    if !existing.trim().is_empty() {
                                        // 既存のスタイルの最後にセミコロンがない場合は追加
                                        if !existing.trim().ends_with(';') {
                                            existing.push(';');
                                        }
                                        existing.push(' ');
                                    }
                                    existing.push_str(&update.styles);
                                    // 新しいスタイルの最後にセミコロンがない場合は追加
                                    if !update.styles.trim().ends_with(';') {
                                        existing.push(';');
                                    }
                                })
                                .or_insert_with(|| {
                                    let mut styles = update.styles.clone();
                                    // 新しいスタイルの最後にセミコロンがない場合は追加
                                    if !styles.trim().ends_with(';') {
                                        styles.push(';');
                                    }
                                    styles
                                });
                        }
                    })
                }

                // APIの応答を処理ハンドラーに渡す
                if let Some(chat_container_styles) = res.chat_container_styles {
                    params.set_chat_container_styles.set(chat_container_styles);
                }
        
                let api_reply = res.message;

                params.set_messages.update(|msgs| {
                    let new_id = msgs.last().map(|m| m.id + 1).unwrap_or(0);
                    msgs.push(Message {
                        id: new_id,
                        text: api_reply.clone(),
                        is_user: false,
                    });
                });

                // 一般的なスタイル変更要求の場合、新しいメッセージ（AIの返信）にもスタイルを適用
                if let Some(element_updates) = &res.change_style_elements {
                    // 全ての要素に同じスタイルが適用されているかチェック（一般的なスタイル変更要求）
                    let all_elements_same_style = element_updates.len() > 1 && 
                        element_updates.iter().all(|update| update.styles == element_updates[0].styles);
                    
                    if all_elements_same_style {
                        // 新しいメッセージ（AIの返信）にも同じスタイルを適用
                        let new_message_id = params.current_messages.len(); // ユーザーメッセージ追加後のID
                        let style_to_apply = element_updates[0].styles.clone();
                        
                        params.set_element_styles.update(|styles| {
                            styles.insert(new_message_id, style_to_apply);
                        });
                    }
                }

            },
            Err(e) => {
                log::error!("API rewuest failed: {:?}", e);
            }
        }
        // ローディング状態をfalseに戻す
        params.set_is_loading.set(false);
    });
}
