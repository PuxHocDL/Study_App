use leptos::ServerFnError;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;

use candle_core::{Device, Tensor};
use candle_examples::token_output_stream::TokenOutputStream;
use candle_transformers::generation::{LogitsProcessor, Sampling};
use candle_transformers::models::quantized_llama::ModelWeights;
use tokenizers::Tokenizer;

pub struct TextGeneration {
    model: ModelWeights,
    tokenizer: TokenOutputStream,
    device: Device,
    logits_processor: LogitsProcessor,
    repeat_penalty: f32,
    repeat_last_n: usize,
}

impl TextGeneration {
    pub fn new(
        model: ModelWeights,
        tokenizer: Tokenizer,
        device: Device,
        seed: u64,
        temp: Option<f64>,
        top_p: Option<f64>,
        top_k: Option<usize>,
        repeat_penalty: f32,
        repeat_last_n: usize,
    ) -> Self {
        let logits_processor = {
            let temperature = temp.unwrap_or(0.);
            let sampling = if temperature <= 0. {
                Sampling::ArgMax
            } else {
                match (top_k, top_p) {
                    (None, None) => Sampling::All { temperature },
                    (Some(k), None) => Sampling::TopK { k, temperature },
                    (None, Some(p)) => Sampling::TopP { p, temperature },
                    (Some(k), Some(p)) => Sampling::TopKThenTopP { k, p, temperature },
                }
            };
            LogitsProcessor::from_sampling(seed, sampling)
        };

        Self {
            model,
            tokenizer: TokenOutputStream::new(tokenizer),
            logits_processor,
            repeat_penalty,
            repeat_last_n,
            device,
        }
    }

    pub fn infer(
        &mut self,
        prompt: &str,
        sample_len: usize,
        tx: &Sender<String>,
    ) -> Result<String, ServerFnError> {
        let runtime = Runtime::new().expect("Tokio runtime creation error");
        let mut inference = String::new();

        self.tokenizer.clear();
        let mut tokens = self
            .tokenizer
            .tokenizer()
            .encode(prompt, true)
            .map_err(|e| ServerFnError::new(e))?
            .get_ids()
            .to_vec();

        let mut generated_tokens = 0usize;
        let eos_token = match self.tokenizer.get_token("</s>") {
            Some(token) => token,
            None => {
                return Err(ServerFnError::new(
                    "Cannot find eos token - </s>".to_string(),
                ))
            }
        };

        println!("\n> Generating tokens");
        let start_gen = std::time::Instant::now();
        for index in 0..sample_len {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let start_pos = tokens.len().saturating_sub(context_size);
            let context = &tokens[start_pos..];

            let input = Tensor::new(context, &self.device)?.unsqueeze(0)?;
            let logits = &mut self.model.forward(&input, start_pos)?;
            let logits = logits.squeeze(0)?.squeeze(0)?;

            let logits = if self.repeat_penalty == 1. {
                logits
            } else {
                let start_at = tokens.len().saturating_sub(self.repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.repeat_penalty,
                    &tokens[start_at..],
                )?
            };

            let next_token = self.logits_processor.sample(&logits)?;
            tokens.push(next_token);
            generated_tokens += 1;

            if next_token == eos_token {
                break;
            }

            if let Some(t) = self.tokenizer.next_token(next_token)? {
                inference.push_str(&t);

                let txc = tx.clone();
                runtime.block_on(async move {
                    txc.send(t).await.expect("Issue sending on channel");
                });
            }
        }

        let dt = start_gen.elapsed();
        if let Some(rest) = self
            .tokenizer
            .decode_rest()
            .map_err(|e| ServerFnError::new(e))?
        {
            inference.push_str(&rest);
            runtime.block_on(async move {
                tx.send(rest).await.expect("Issue sending on channel");
            });
        }

        println!(
            "> {generated_tokens} tokens generated ({:.2} token/s)",
            generated_tokens as f64 / dt.as_secs_f64(),
        );

        Ok(inference)
    }
}