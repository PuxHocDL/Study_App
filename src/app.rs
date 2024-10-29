use std::cell::RefCell;
use std::rc::Rc;

use futures::stream::SplitSink;
use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;
use components::chat_area::OtherChatArea;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (conversation, set_conversation) = create_signal(Conversation::new());
    let (current_chatbot, set_current_chatbot) = create_signal("chatbot_1".to_string());

    use futures::{SinkExt, StreamExt};
    use gloo_net::websocket::{futures::WebSocket, Message::Text};

    let client: Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>> =
        Default::default();

    // Chỉnh sửa closure của change_chatbot để trả về Future
    let change_chatbot = create_action(move |new_chatbot: &String| {
        let new_chatbot = new_chatbot.clone();
        async move {
            set_current_chatbot(new_chatbot);
        }
    });

    let client_clone = client.clone();
    create_effect(move |_| {
        let location = web_sys::window().unwrap().location();
        let hostname = location
            .hostname()
            .expect("Failed to retrieve origin hostname");
        let ws_url = format!("ws://{hostname}:3000/ws");

        let connection =
            WebSocket::open(&ws_url.to_string()).expect("failed to establish WebSocket connection");

        let (sender, mut recv) = connection.split();
        spawn_local(async move {
            while let Some(msg) = recv.next().await {
                match msg {
                    Ok(Text(msg)) => {
                        set_conversation.update(move |c| {
                            c.messages.last_mut().unwrap().text.push_str(&msg);
                        });
                    }
                    _ => {
                        break;
                    }
                }
            }
        });

        *client_clone.borrow_mut() = Some(sender);
    });

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        let client2 = client.clone();
        let msg = new_message.to_string();
        async move {
            client2
                .borrow_mut()
                .as_mut()
                .unwrap()
                .send(Text(msg.to_string()))
                .await
                .map_err(|_| ServerFnError::ServerError("WebSocket issue".to_string()))
        }
    });

    create_effect(move |_| {
        if send.input().get().is_some() {
            let model_message = Message {
                text: String::new(),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/candlemist.css"/>
        <Title text="CandleMist"/>

        // Thêm các nút điều hướng
        <div class="nav-buttons">
            <button on:click=move |_| change_chatbot.dispatch("chatbot_1".to_string())>
                "Chatbot 1"
            </button>
            <button on:click=move |_| change_chatbot.dispatch("chatbot_2".to_string())>
                "Chatbot 2"
            </button>
        </div>

        // Điều hướng giữa các chatbot
        { move || match current_chatbot.get().as_str() {
            "chatbot_1" => view! {
                <ChatArea conversation/>
                <TypeArea send/>
            }.into_view(),
            "chatbot_2" => view! {
                <OtherChatArea/> // Chatbot khác
            }.into_view(),
            _ => view! {
                <div>"Không có chatbot nào được chọn"</div>
            }.into_view(),
        }}
    }
}