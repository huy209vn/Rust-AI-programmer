//! Model inference and weight loading

use crate::model::{Qwen2Config, Qwen2ForCausalLM, KeyValueCache};
use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};
use safetensors::SafeTensors;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Load Qwen2 model from Safetensors weights (handles sharded models)
///
/// # Arguments
/// * `model_dir` - Path to the directory containing safetensors files
/// * `device` - Device to load the model on
///
/// # Returns
/// The loaded model ready for inference
pub fn load_model<B: Backend>(
    model_dir: &str,
    device: &B::Device,
) -> Result<Qwen2ForCausalLM<B>, String> {
    println!("Loading Strand-Rust-Coder-14B-v1 model...");

    // Initialize model configuration
    let config = Qwen2Config::strand_rust_coder_14b();

    // Create model with random weights (we'll overwrite these)
    let model = config.init(device);

    // For now, just return the model with random weights
    // TODO: Implement actual weight loading from safetensors
    println!("⚠️  WARNING: Loading model with random weights (weight loading not yet implemented)");
    println!("Model structure created successfully!");

    Ok(model)
}

/// Generate text from a prompt using the model
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
        let input = if cache[0].len() == 0 {
            // First pass: use full input
            generated.clone()
        } else {
            // Subsequent passes: only use last generated token
            let seq_len = generated.dims()[1];
            generated.clone().slice([0..batch_size, seq_len - 1..seq_len])
        };

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

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::NdArray;

    type Backend = NdArray<f32>;

    #[test]
    fn test_model_init() {
        let device = Default::default();
        let config = Qwen2Config::strand_rust_coder_14b();
        let model = config.init::<Backend>(&device);

        // Test that model initializes without panic
        let cache = model.init_cache(&config, 1, &device);
        assert_eq!(cache.len(), config.num_hidden_layers);
    }
}
