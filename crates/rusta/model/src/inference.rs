//! Model inference and weight loading (Safetensors)

use crate::model::{Qwen2Config, Qwen2ForCausalLM, KeyValueCache};
use burn::tensor::{backend::Backend, Int, Tensor};
use burn::module::Module;

use burn_store::{SafetensorsStore, ModuleSnapshot};

use anyhow::{Result, anyhow};
use std::path::PathBuf;

pub fn load_model<B: Backend>(
    model_dir: &str,
    device: &B::Device,
) -> Result<Qwen2ForCausalLM<B>> {

    // 1. Collect shard paths
    let shard_paths = find_shards(model_dir);
    if shard_paths.is_empty() {
        return Err(anyhow!("No safetensors shard files found"));
    }

    println!("🔍 Found {} shards:", shard_paths.len());
    for p in &shard_paths {
        println!("  • {}", p.display());
    }

    // 2. Initialize empty model
    let config = Qwen2Config::strand_rust_coder_14b();
    let mut model = config.init(device);

    // 3. Sequentially load each shard into the SAME model
    // NOTE: The safetensors files must have weights transposed to match Burn's format
    // Run scripts/transpose_safetensors.py first if you get shape mismatch errors
    for shard_path in shard_paths {
        println!("📦 Loading shard: {}", shard_path.display());

        let mut store = SafetensorsStore::from_file(shard_path)
            .allow_partial(true);

        model
            .load_from(&mut store)
            .map_err(|e| anyhow!("Failed loading shard: {}", e))?;
    }

    println!("✅ All shards loaded successfully.");
    Ok(model)
}

fn find_shards(dir: &str) -> Vec<PathBuf> {
    let mut out = vec![];

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "safetensors").unwrap_or(false) {
                out.push(path);
            }
        }
    }

    out.sort_by(|a, b| a.file_name().unwrap().cmp(b.file_name().unwrap()));
    out
}

///
/// # Arguments
/// * `model` - The loaded Qwen2 model
/// * `config` - Model configuration
/// * `input_ids` - Tokenized input [batch_size, seq_len]
/// * `max_new_tokens` - Maximum number of tokens to generate
/// * `temperature` - Sampling temperature (1.0 = neutral, lower = more deterministic)
/// * `device` - Device to run inference on
///
/// # Returns
/// Generated token IDs [batch_size, seq_len + max_new_tokens]
pub fn generate<B: Backend>(
    model: &Qwen2ForCausalLM<B>,
    config: &Qwen2Config,
    input_ids: Tensor<B, 2, Int>,
    max_new_tokens: usize,
    temperature: f32,
    device: &B::Device,
) -> Tensor<B, 2, Int> {
    let [batch_size, _initial_seq_len] = input_ids.dims();

    // Initialize KV cache
    let mut cache = model.init_cache(config, batch_size, device);

    // Start with the input tokens
    let mut generated = input_ids.clone();

    // Generate tokens autoregressively
    for _ in 0..max_new_tokens {
        // Get the last token (or all tokens on first pass)
        let seq_len = generated.dims()[1];
        let input = generated.clone().slice([0..batch_size, seq_len - 1..seq_len]);
        // Forward pass
        let logits = model.forward(input, &mut cache);

        // Get logits for the last position [batch_size, 1, vocab_size] -> [batch_size, vocab_size]
        let [_b, seq_len, vocab_size] = logits.dims();
        let last_logits = logits.slice([0..batch_size, seq_len - 1..seq_len, 0..vocab_size]);
        let last_logits = last_logits.squeeze::<2>(); // Squeeze to [batch_size, vocab_size]

        // Apply temperature scaling
        let scaled_logits = last_logits.div_scalar(temperature);

        // Sample next token (greedy for now - TODO: add top-p, top-k sampling)
        let next_token = scaled_logits.argmax(1);

        // Append to generated sequence
        generated = Tensor::cat(vec![generated, next_token.unsqueeze()], 1);

        // TODO: Check for EOS token and break early
    }

    generated
}

/// Initialize a KV cache for inference
pub fn init_cache<B: Backend>(
    config: &Qwen2Config,
    batch_size: usize,
    device: &B::Device,
) -> Vec<KeyValueCache<B>> {
    let head_dim = config.hidden_size / config.num_attention_heads;
    (0..config.num_hidden_layers)
        .map(|_| {
            KeyValueCache::new(
                batch_size,
                config.num_key_value_heads,
                config.max_position_embeddings,
                head_dim,
                device,
            )
        })
        .collect()
}



