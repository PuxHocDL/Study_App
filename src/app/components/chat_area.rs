use leptos::{html::Div, *};

use crate::model::conversation::Conversation;

const USER_MESSAGE_CLASS: &str = "max-w-lg p-4 mb-5 self-end";
const USER_MESSAGE_COLOURS: &str = "bg-gradient-to-r from-user_d to-user_l text-white";

const MODEL_MESSAGE_CLASS: &str = "max-w-lg p-4 mb-5 self-start";
const MODEL_MESSAGE_COLOURS: &str = "bg-gradient-to-r from-model_d to-model_l text-white";

const CHAT_AREA_CLASS: &str = "h-screen pb-24 w-full flex flex-col overflow-y-auto p-5";
const CHAT_AREA_COLOURS: &str = "bg-background";

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let user_message_class: Signal<String> =
        Signal::derive(move || format!("{USER_MESSAGE_CLASS} {USER_MESSAGE_COLOURS}"));

    let model_message_class: Signal<String> =
        Signal::derive(move || format!("{MODEL_MESSAGE_CLASS} {MODEL_MESSAGE_COLOURS}"));

    let chat_area_class: Signal<String> =
        Signal::derive(move || format!("{CHAT_AREA_CLASS} {CHAT_AREA_COLOURS}"));

    let chat_div_ref = create_node_ref::<Div>();
    create_effect(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        <div class=chat_area_class.get() node_ref=chat_div_ref>
            {move || {
                conversation
                    .get()
                    .messages
                    .iter()
                    .map(move |message| {
                        let message_class_str = if message.user {
                            user_message_class.get()
                        } else {
                            model_message_class.get()
                        };
                        view! {
                            <div
                                class=message_class_str
                                inner_html=markdown::to_html(&message.text)
                            ></div>
                        }
                    })
                    .collect::<Vec<_>>()
            }}

        </div>
    }
}
