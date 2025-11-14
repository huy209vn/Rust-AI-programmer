//! Text generation example for Strand-Rust-Coder-14B-v1
//!
//! This example demonstrates GPU-accelerated text generation using:
//! 1. LibTorch backend with CUDA
//! 2. Tokenizer for encoding/decoding
//! 3. Autoregressive generation with KV-cache
//!
//! Prerequisites:
//! - CUDA-capable GPU (16GB+ VRAM)
//! - LibTorch installed (set LIBTORCH environment variable)
//! - LIBTORCH_USE_PYTORCH=1 environment variable
//!
//! Usage:
//! ```bash
//! cargo run --example text_generation --release -- \
//!     --model-path "C:/Users/PC/.cache/huggingface/hub/models--Fortytwo-Network--Strand-Rust-Coder-14B-v1/snapshots/0b9a97c5ab89f9780c95356cc2ea121eb434372e" \
//!     --prompt "fn fibonacci(n: u32) -> u32 {"
//! ```

use burn::tensor::{backend::Backend, Int, Tensor};
use rusta_model::{generate, load_model, Qwen2Config, Qwen2Tokenizer};

// Using LibTorch backend for CUDA GPU acceleration
type MyBackend = burn_tch::LibTorch<f32>;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse arguments
    let mut model_path = None;
    let mut prompt = "fn main() {".to_string();
    let mut max_new_tokens = 100;
    let mut temperature = 0.7;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--model-path" => {
                model_path = Some(args[i + 1].clone());
                i += 2;
            }
            "--prompt" => {
                prompt = args[i + 1].clone();
                i += 2;
            }
            "--max-tokens" => {
                max_new_tokens = args[i + 1].parse().expect("Invalid max-tokens");
                i += 2;
            }
            "--temperature" => {
                temperature = args[i + 1].parse().expect("Invalid temperature");
                i += 2;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                print_help();
                std::process::exit(1);
            }
        }
    }

    let model_path = model_path.expect("--model-path is required. Use --help for usage info.");

    println!("=== Strand-Rust-Coder-14B-v1 Text Generation ===\n");
    println!("Model path: {}", model_path);
    println!("Prompt: {}", prompt);
    println!("Max new tokens: {}", max_new_tokens);
    println!("Temperature: {}\n", temperature);

    // Initialize CUDA device (GPU 0)
    println!("Initializing CUDA device...");
    let device = burn_tch::LibTorchDevice::Cuda(0);
    println!("Using device: CUDA:0\n");

    // Load tokenizer
    println!("Loading tokenizer...");
    let tokenizer = Qwen2Tokenizer::from_file(&model_path)
        .expect("Failed to load tokenizer. Make sure tokenizer.json exists in the model directory.");

    // Encode the prompt
    println!("Encoding prompt...");
    let input_ids = tokenizer
        .encode(&prompt, false)
        .expect("Failed to encode prompt");

    println!("Prompt tokens: {:?}", input_ids);
    println!("Number of prompt tokens: {}\n", input_ids.len());

    // Convert to tensor [batch_size=1, seq_len]
    let input_tensor: Tensor<MyBackend, 2, Int> = Tensor::from_ints(
        input_ids
            .iter()
            .map(|&id| id as i64)
            .collect::<Vec<_>>()
            .as_slice(),
        &device,
    )
    .reshape([1, input_ids.len()]);

    // Load model
    println!("Loading model (this may take a few minutes for 14B parameters)...");
    let model = load_model::<MyBackend>(&model_path, &device)
        .expect("Failed to load model. Make sure model.safetensors.index.json and weight files exist.");

    // Generate
    println!("Generating text...\n");
    println!("--- OUTPUT ---");
    print!("{}", prompt); // Print the prompt first

    let config = Qwen2Config::strand_rust_coder_14b();
    let output_tensor = generate(&model, &config, input_tensor, max_new_tokens, temperature, &device);

    // Convert output tensor to Vec<u32>
    let output_ids: Vec<u32> = output_tensor
        .into_data()
        .to_vec::<i64>()
        .expect("Failed to convert tensor to vec")
        .iter()
        .skip(input_ids.len()) // Skip the prompt tokens
        .map(|&id| id as u32)
        .collect();

    // Decode and print
    let generated_text = tokenizer
        .decode(&output_ids, true)
        .expect("Failed to decode output");

    println!("{}", generated_text);
    println!("\n--- END OUTPUT ---\n");
}

fn print_help() {
    println!("Strand-Rust-Coder-14B-v1 Text Generation (GPU/CUDA)");
    println!("\nUsage:");
    println!("  cargo run --example text_generation --release -- [OPTIONS]");
    println!("\nOptions:");
    println!("  --model-path <PATH>    Path to model directory (required)");
    println!("  --prompt <TEXT>        Input prompt (default: 'fn main() {{')");
    println!("  --max-tokens <NUM>     Maximum tokens to generate (default: 100)");
    println!("  --temperature <NUM>    Sampling temperature (default: 0.7)");
    println!("  --help, -h             Show this help message");
    println!("\nPrerequisites:");
    println!("  - CUDA-capable GPU (16GB+ VRAM recommended)");
    println!("  - LibTorch installed (set LIBTORCH environment variable)");
    println!("  - LIBTORCH_USE_PYTORCH=1 environment variable");
    println!("\nExample:");
    println!(r#"  cargo run --example text_generation --release -- \"#);
    println!(r#"    --model-path "C:/Users/PC/.cache/huggingface/hub/models--Fortytwo-Network--Strand-Rust-Coder-14B-v1/snapshots/0b9a97c5ab89f9780c95356cc2ea121eb434372e" \"#);
    println!(r#"    --prompt "fn fibonacci(n: u32) -> u32 {{" \"#);
    println!(r#"    --max-tokens 200"#);
}
