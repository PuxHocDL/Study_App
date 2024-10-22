use anyhow::Error as E;
use candle_core::{quantized::gguf_file, utils::metal_is_available, Device};
use candle_transformers::models::quantized_llama::ModelWeights;
use dotenv::{dotenv, var};
use tokenizers::Tokenizer;

pub fn model_loader() -> Result<(ModelWeights, Tokenizer, Device), E> {
    dotenv().ok();

    let model_path = var("MODEL_PATH").expect("MODEL_PATH is not set");
    let mut file = std::fs::File::open(&model_path)?;
    let model = gguf_file::Content::read(&mut file).map_err(|e| e.with_path(model_path))?;

    let mut total_size_in_bytes = 0;
    for (_, tensor) in model.tensor_infos.iter() {
        let elem_count = tensor.shape.elem_count();
        total_size_in_bytes +=
            elem_count * tensor.ggml_dtype.type_size() / tensor.ggml_dtype.block_size();
    }

    let device = if metal_is_available() {
        Device::new_metal(0)?
    } else {
        Device::cuda_if_available(0)?
    };

    println!(
        "\n> Loading {:?} tensors - {}",
        model.tensor_infos.len(),
        &format_size(total_size_in_bytes),
    );
    let model = ModelWeights::from_gguf(model, &mut file, &device)?;
    println!("> Successfully loaded tensors ✓");

    let tokenizer_path = var("TOKENIZER_PATH").expect("TOKENIZER_PATH is not set");
    let tokenizer_filename = std::path::PathBuf::from(tokenizer_path);

    println!("\n> Loading tokenizer");
    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
    println!("> Successfully loaded tokenizer ✓\n");

    Ok((model, tokenizer, device))
}

fn format_size(size_in_bytes: usize) -> String {
    if size_in_bytes < 1_000 {
        format!("{}B", size_in_bytes)
    } else if size_in_bytes < 1_000_000 {
        format!("{:.2}KB", size_in_bytes as f64 / 1e3)
    } else if size_in_bytes < 1_000_000_000 {
        format!("{:.2}MB", size_in_bytes as f64 / 1e6)
    } else {
        format!("{:.2}GB", size_in_bytes as f64 / 1e9)
    }
}
