# Chat_bot

- Chatbot đơn giản được xây dựng bằng Rust, ở cả phần frontend và backend.
- Sử dụng các thư viện: candle, leptos, actix, tokio và TailwindCSS.
- Sử dụng các mô hình Mistral 7B Instruct v0.1 GGUF đã được lượng tử hóa.

### Source

- Đây là một nhánh từ [MoonKraken/rusty_llama](https://github.com/MoonKraken/rusty_llama) của [Code to the Moon](https://www.youtube.com/watch?v=vAjle3c9Xqc).
- Chatbot này sử dụng các mô hình Mistral GGUF và framework [huggingface/candle](https://github.com/huggingface/candle), bao gồm thư viện candle-transformers, trong khi phiên bản gốc sử dụng các mô hình GGML và thư viện [rustformers/llm](https://github.com/rustformers/llm).
- Phần frontend có một số thay đổi về giao diện, nhưng cấu trúc tổng thể vẫn giữ nguyên.
- Màu sắc được lấy từ bảng màu Tokyo Night.

## Hướng dẫn cài đặt

### Công cụ Rust

Bạn cần sử dụng toolchain Rust phiên bản nightly, cài đặt target `wasm32-unknown-unknown`, cũng như các công cụ `trunk` và `cargo-leptos`:

```bash
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
cargo install trunk (optional!)
```

### Hardware

- Về CUDA, thêm thuộc tính CUDA vào Cargo.toml:
```bash
candle-core = { git = "https://github.com/huggingface/candle.git", version = "0.6.0", optional = true, features = ["cuda"] }
```
- Đối với Metal, thêm tính năng metal cho candle_core trong file Cargo.toml.
- Đối với thư viện toán của Intel’s oneAPI Math Kernel Library, thêm tính năng mkl cho candle_core trong file Cargo.toml.

### Mô hình
Tải về bất kỳ mô hình **Mistral 7B Instruct** v0.1 GGUF nào và thiết lập biến môi trường MODEL_PATH trong file .env.
Mô hình đã kiểm tra
- mistral-7b-instruct-v0.1.Q4_K_M.gguf
- dolphin-2.6-mistral-7b.Q4_K_M.gguf

Tải về file tokenizer.json và thiết lập biến môi trường TOKENIZER_PATH trong file .env.
+ Mistral-7B-v0.1/tokenizer.json

### TailwindCSS
Install TailwindCSS with 
```bash
npm install -D tailwindcss
```
### Thực thi

1.
```bash
git clone https://github.com/PuxHocDL/Study_App.git
cd Study_App
```

2.
```bash
npx tailwindcss -i ./input.css -o ./style/output.css
```

3.
```bash
cargo leptos serve --release
```

4.Mở trang local để chạy:
[http://localhost:3000/?](http://localhost:3000/?)
