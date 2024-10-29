use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::collections::VecDeque;
        use std::sync::Arc;

        use actix_web::{web, web::Payload, Error, HttpRequest, HttpResponse};
        use actix_ws::Message as Msg;
        use futures::stream::StreamExt;

        use candle_core::Device;
        use candle_transformers::models::quantized_llama::ModelWeights;
        use tokenizers::Tokenizer;

        pub mod loader;
        mod textgen;

        use textgen::*;

        const CHAT_TEMPLATE: &str = "[INST] Could you please assist me by answering some questions? Be brief, focused and follow my instructions clearly. [/INST]Sure, I will answer your questions to the best of my abilities.</s>";
        const MAX_HISTORY: usize = 10;

        pub async fn ws(
            req: HttpRequest,
            body: Payload,
            model: web::Data<ModelWeights>,
            tokenizer: web::Data<Tokenizer>,
            device: web::Data<Device>,
        ) -> Result<HttpResponse, Error> {
            let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;

            use std::sync::Mutex;
            use tokio::sync::mpsc;

            let (send_inference, mut recieve_inference) = mpsc::channel::<String>(50);

            let mdl: ModelWeights = model.get_ref().clone();
            let tkn: Tokenizer = tokenizer.get_ref().clone();
            let dvc: Device = device.get_ref().clone();

            let sess = Arc::new(Mutex::new(session));
            let sess_cloned = sess.clone();

            actix_rt::spawn(async move {
                let (send_new_prompt, recieve_new_prompt) = std::sync::mpsc::channel::<String>();

                std::thread::spawn(move || {
                    let mut pipeline = TextGeneration::new(
                        mdl,
                        tkn,
                        dvc,
                        fastrand::u64(..),
                        Some(0.1),
                        None,
                        None,
                        1.,
                        64,
                    );

                    let mut history = VecDeque::new();

                    for new_prompt in recieve_new_prompt {
                        let prompt = format_prompt(&new_prompt, &history);
                        let inference = pipeline
                            .infer(&prompt, 250, &send_inference)
                            .expect("Error in inferencing");

                        history.push_back((new_prompt, inference));

                        if history.len() >= MAX_HISTORY {
                            history.pop_front();
                        }
                    }
                });

                while let Some(Ok(msg)) = msg_stream.next().await {
                    match msg {
                        Msg::Ping(bytes) => {
                            let res = sess_cloned.lock().unwrap().pong(&bytes).await;
                            if res.is_err() {
                                return;
                            }
                        }
                        Msg::Text(s) => {
                            let _ = send_new_prompt.send(s.to_string());
                        }
                        _ => break,
                    }
                }
            });

            actix_rt::spawn(async move {
                while let Some(message) = recieve_inference.recv().await {
                    sess.lock()
                        .unwrap()
                        .text(message)
                        .await
                        .expect("Issue sending on websocket");
                }
            });

            Ok(response)
        }

        fn format_prompt(prompt: &String, history: &VecDeque<(String, String)>) -> String {
            let history: String = history
                .iter()
                .map(|(user_prompt, model_response)| {
                    format!("[INST] {user_prompt} [/INST]{model_response}</s>")
                })
                .collect::<Vec<String>>()
                .join(" ");

            let prompt = format!("[INST] {prompt} [/INST]");

            format!("{CHAT_TEMPLATE} {history} {prompt}")
        }
    }
}