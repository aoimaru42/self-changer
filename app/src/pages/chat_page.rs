use crate::api_client::{send_message_to_api, ApiCallParams};
use common::DynamicElementData;
use leptos::ev::SubmitEvent;
use leptos::prelude::signal as leptos_signal;
use leptos::prelude::*;
use std::collections::HashMap;

// メッセージのデータを保持する構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub id: usize,
    pub text: String,
    pub is_user: bool, // true: ユーザー, false: AI/システム
}

/// チャットUIのホームページをレンダリングします
#[component]
pub fn ChatPage() -> impl IntoView {
    // メッセージリストの状態を管理
    let (messages, set_messages) = leptos_signal(vec![Message {
        id: 0,
        text: "こんにちは！self changerチャットへようこそ。".to_string(),
        is_user: false,
    }]);

    // 新しいメッセージ入力フォームの状態を管理
    let (new_message_text, set_new_message_text) = leptos_signal("".to_string());

    // API呼び出しの状態を管理
    let (is_loading, set_is_loading) = leptos_signal(false);

    // チャットコンテナ全体の状態を管理（CSSプロパティ形式）
    let (chat_container_styles, set_chat_container_styles) = leptos_signal("".to_string());

    // 各メッセージIDに紐づく動的要素のリストを管理
    let (dynamic_elements, set_dynamic_elements) =
        leptos_signal(HashMap::<usize, Vec<DynamicElementData>>::new());

    // 動的要素のスタイルを集中管理するHashMap（CSSプロパティ文字列を保持）
    let (element_styles, set_element_styles) = leptos_signal(HashMap::<usize, String>::new());

    // 初期の追記クラスは空（ベースクラスは描画時に付与）
    let initial_styles: HashMap<usize, String> = HashMap::new();
    set_element_styles.set(initial_styles);

    // フォームの送信時に実行される関数
    let on_submit = move |ev: SubmitEvent| {
        // デフォルトのフォーム動作（ページの再読み込み）を止める
        ev.prevent_default();
        let message = new_message_text.get_untracked();
        let trimmed_message = message.trim().to_string();

        if !trimmed_message.is_empty() {
            let next_id = messages
                .get_untracked()
                .last()
                .map(|m| m.id + 1)
                .unwrap_or(0);
            set_messages.update(|msgs| {
                msgs.push(Message {
                    id: next_id,
                    text: message.clone(),
                    is_user: true,
                });
            });

            // 送信後、入力フィールドを空にする
            set_new_message_text.set("".to_string());

            // API処理
            send_message_to_api(ApiCallParams {
                user_message: trimmed_message,
                anchor_message_id: next_id,
                current_messages: messages.get(),
                set_is_loading,
                set_messages,
                set_chat_container_styles,
                set_dynamic_elements,
                set_element_styles,
            });
        }
    };

    let on_refresh = move |_| {
        // メッセージリストを初期状態に戻す
        set_messages.set(vec![Message {
            id: 0,
            text: "こんにちは！チャットへようこそ。".to_string(),
            is_user: false,
        }]);
        // 動的要素もクリア（HashMap に変更したため）
        set_dynamic_elements.set(HashMap::new());
        // スタイルマップを初期状態に戻す
        set_element_styles.set(HashMap::new());
        // コンテナのスタイルを初期状態に戻す
        set_chat_container_styles.set("".to_string());
    };

    view! {
        <div class="main-container">
            // ローディングオーバーレイ
            <Show when=move || is_loading.get()>
                <div class="loading-overlay">
                    <div class="loading-text">
                        "読み込み中..."
                    </div>
                </div>
            </Show>
            // リフレッシュボタン
            <button
                on:click=on_refresh
                class="refresh-button"
            >
                <svg xmlns="http://www.w3.org/2000/svg" class="refresh-icon" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.76 0 3.32.74 4.46 1.96L13 11h7V4z" />
                </svg>
            </button>
            <div class="chat-container" style=move || chat_container_styles.get()>
                // メッセージ履歴表示エリア
                <div class="messages-area">
                    <For
                        // RwSignalから.get()で値を取得
                        each=move || messages.get()
                        key=|msg| msg.id
                        children=move |msg| {
                            // ベーススタイル + 動的スタイルをマージ
                            let msg_styles = move || {
                                let base_styles = if msg.is_user {
                                    "padding: 12px 16px; border-radius: 18px; border-bottom-right-radius: 4px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1); max-width: 85%; text-align: left; background-color: #dbeafe; color: #1f2937;"
                                } else {
                                    "padding: 12px 16px; border-radius: 18px; border-bottom-left-radius: 4px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1); max-width: 85%; text-align: left; background-color: #e5e7eb; color: #1f2937;"
                                };
                                // element_stylesの変更を明示的に追跡（リアクティブに更新される）
                                let extras_map = element_styles.get();
                                if let Some(extra) = extras_map.get(&msg.id) {
                                    if extra.trim().is_empty() {
                                        base_styles.to_string()
                                    } else {
                                        // 動的スタイルをベーススタイルの後に追加（後から適用されるため優先される）
                                        format!("{} {}", base_styles, extra)
                                    }
                                } else {
                                    base_styles.to_string()
                                }
                            };
                            view! {
                                <div class=move || {
                                    format!("message-item {}", if msg.is_user { "justify-end" } else { "justify-start" })
                                }>
                                    // アイコン
                                    {(!msg.is_user).then(|| view! {
                                        <div class="message-icon">
                                            {"🤖"}
                                        </div>
                                    })}

                                    // メッセージの吹き出し
                                    <div style=msg_styles>
                                        <p class="message-text">{msg.text.clone()}</p>
                                    </div>
                                </div>
                                // このメッセージ直後に紐づいた動的要素を描画
                                {move || {
                                    let map = dynamic_elements.get();
                                    let list = map.get(&msg.id).cloned().unwrap_or_default();
                                    if list.is_empty() {
                                        ().into_any()
                                    } else {
                                        view! {
                                            <For
                                                each=move || list.clone()
                                                key=|elem| elem.id
                                                children=move |elem| {
                                                    let styles = elem.styles.clone().unwrap_or_default();
                                                    let text = elem.text.clone().unwrap_or_default();
                                                    let attrs = elem.attributes.clone().unwrap_or_default();
                                                    let child: AnyView = match elem.tag.as_str() {
                                                        "div" => view! { <div>{text}</div> }.into_any(),
                                                        "p" => view! { <p>{text}</p> }.into_any(),
                                                        "span" => view! { <span>{text}</span> }.into_any(),
                                                        "button" => view! { <button>{text}</button> }.into_any(),
                                                        "a" => {
                                                            let href = attrs.get("href").cloned().unwrap_or_default();
                                                            view! { <a href=href>{text}</a> }.into_any()
                                                        },
                                                        "img" => {
                                                            let src = attrs.get("src").cloned().unwrap_or_default();
                                                            let alt = attrs.get("alt").cloned().unwrap_or_default();
                                                            view! { <img src=src alt=alt/> }.into_any()
                                                        },
                                                        "input" => {
                                                            let input_type = attrs.get("type").cloned().unwrap_or_else(|| "text".to_string());
                                                            let placeholder = attrs.get("placeholder").cloned().unwrap_or_default();
                                                            let value = attrs.get("value").cloned().unwrap_or_default();
                                                            view! { <input r#type=input_type placeholder=placeholder value=value/> }.into_any()
                                                        },
                                                        _ => view! { <div>{text}</div> }.into_any(),
                                                    };
                                                    view! { <div class="dynamic-element" style=styles>{child}</div> }.into_any()
                                                }
                                            />
                                        }.into_any()
                                    }
                                }}
                            }
                        }
                    />
                </div>
                // 入力フォームとボタン
                <form
                    on:submit=on_submit
                    class="input-form"
                >
                    <input
                        type="text"
                        prop:value=new_message_text
                        on:input=move |ev| {
                            set_new_message_text(event_target_value(&ev));
                        }
                        placeholder="メッセージを入力..."
                        class="input-field"
                    />
                    <button
                        type="submit"
                        class="send-button"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="send-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                        </svg>
                    </button>
                </form>
            </div>
        </div>
    }
}
