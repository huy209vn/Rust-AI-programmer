//! Rusta CLI - Inference loop for Strand-Rust-Coder-14B-v1
//!
//! Simple interactive inference loop using native CUDA.

use burn::tensor::{backend::Backend, Int, Tensor};
use clap::Parser;
use rusta_model::{generate, load_model, Qwen2Config, Qwen2Tokenizer};
use std::io::{self, Write};

// Native CUDA backend via CubeCL
type MyBackend = burn_cuda::Cuda<f32>;

#[derive(Parser, Debug)]
#[command(name = "rusta")]
#[command(about = "Rusta - Rust AI Programmer", long_about = None)]
struct Args {
    /// Path to model directory
    #[arg(short, long)]
    model_path: String,

    /// Maximum tokens to generate per prompt
    #[arg(short = 't', long, default_value = "200")]
    max_tokens: usize,

    /// Sampling temperature (0.1 = deterministic, 1.0 = creative)
    #[arg(long, default_value = "0.7")]
    temperature: f32,

    /// Single prompt mode (non-interactive)
    #[arg(short, long)]
    prompt: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("🦀 Rusta - Rust AI Programmer");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Initialize CUDA device
    println!("⚡ Initializing CUDA device...");
    let device = Default::default();
    println!("✓ Using CUDA (native via CubeCL)\n");

    // Load tokenizer
    println!("📚 Loading tokenizer...");
    let tokenizer = Qwen2Tokenizer::from_file(&args.model_path)
        .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;
    println!("✓ Tokenizer loaded\n");

    // Load model
    println!("🧠 Loading Strand-Rust-Coder-14B-v1 model...");
    println!("   (This may take 1-2 minutes for 14B parameters)");
    let model = load_model::<MyBackend>(&args.model_path, &device)
        .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;
    println!("✓ Model loaded successfully!\n");

    let config = Qwen2Config::strand_rust_coder_14b();

    // Single prompt mode or interactive loop
    if let Some(prompt) = args.prompt {
        generate_response(&model, &config, &tokenizer, &prompt, args.max_tokens, args.temperature, &device)?;
    } else {
        interactive_loop(&model, &config, &tokenizer, args.max_tokens, args.temperature, &device)?;
    }

    Ok(())
}

fn interactive_loop<B: Backend>(
    model: &rusta_model::Qwen2ForCausalLM<B>,
    config: &Qwen2Config,
    tokenizer: &Qwen2Tokenizer,
    max_tokens: usize,
    temperature: f32,
    device: &B::Device,
) -> anyhow::Result<()> {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Interactive Mode");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Enter Rust code prompts. Type 'exit' or 'quit' to stop.");
    println!("Type 'clear' to clear screen.\n");

    loop {
        print!("\n🦀 > ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let prompt = input.trim();

        if prompt.is_empty() {
            continue;
        }

        match prompt.to_lowercase().as_str() {
            "exit" | "quit" => {
                println!("\nGoodbye! 👋");
                break;
            }
            "clear" | "cls" => {
                print!("\x1B[2J\x1B[1;1H");
                continue;
            }
            _ => {}
        }

        generate_response(model, config, tokenizer, prompt, max_tokens, temperature, device)?;
    }

    Ok(())
}

fn generate_response<B: Backend>(
    model: &rusta_model::Qwen2ForCausalLM<B>,
    config: &Qwen2Config,
    tokenizer: &Qwen2Tokenizer,
    prompt: &str,
    max_tokens: usize,
    temperature: f32,
    device: &B::Device,
) -> anyhow::Result<()> {
    // Encode prompt
    let input_ids = tokenizer.encode(prompt, false)
        .map_err(|e| anyhow::anyhow!("Failed to encode prompt: {}", e))?;
    println!("\n📝 Prompt tokens: {}", input_ids.len());

    // Convert to tensor [batch_size=1, seq_len]
    let input_tensor: Tensor<B, 2, Int> = Tensor::<B, 1, Int>::from_ints(
        input_ids
            .iter()
            .map(|&id| id as i64)
            .collect::<Vec<_>>()
            .as_slice(),
        device,
    )
    .reshape([1, input_ids.len()]);

    // Generate
    println!("🔮 Generating...\n");
    println!("────────────────────────────────────────────────────");
    print!("{}", prompt);
    io::stdout().flush()?;

    let output_tensor = generate(model, config, input_tensor, max_tokens, temperature, device);

    // Decode
    let output_ids: Vec<u32> = output_tensor
        .into_data()
        .to_vec::<i64>()
        .unwrap()
        .iter()
        .skip(input_ids.len())
        .map(|&id| id as u32)
        .collect();

    let generated_text = tokenizer.decode(&output_ids, true)
        .map_err(|e| anyhow::anyhow!("Failed to decode output: {}", e))?;
    println!("{}", generated_text);
    println!("────────────────────────────────────────────────────");
    println!("\n✓ Generated {} tokens", output_ids.len());

    Ok(())
}
