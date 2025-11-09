use anyhow::{Context, Result};
use burn::tensor::{backend::Backend, Tensor, TensorData};
use safetensors::SafeTensors;
use std::{collections::HashMap, path::Path};

use crate::{config::Qwen2Config, Qwen2ForCausalLM};

/// Loads pretrained Qwen2.5 weights from safetensors format
pub struct WeightLoader {
    /// Mapping from safetensors keys to our weight names
    weight_map: HashMap<String, Vec<u8>>,
}

impl WeightLoader {
    /// Load weights from a safetensors file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let bytes = std::fs::read(path.as_ref())
            .context("Failed to read safetensors file")?;

        Self::from_bytes(&bytes)
    }

    /// Load weights from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let tensors = SafeTensors::deserialize(bytes)
            .context("Failed to deserialize safetensors")?;

        let mut weight_map = HashMap::new();

        // Extract all tensors
        for name in tensors.names() {
            let view = tensors
                .tensor(&name)
                .context(format!("Failed to get tensor: {}", name))?;

            // Store tensor data
            weight_map.insert(name.to_string(), view.data().to_vec());
        }

        Ok(Self { weight_map })
    }

    /// Apply weights to a Qwen2ForCausalLM model
    ///
    /// This maps HuggingFace weight names to our Burn model structure
    pub fn load_into_model<B: Backend>(
        &self,
        _model: &mut Qwen2ForCausalLM<B>,
        _device: &B::Device,
    ) -> Result<()> {
        tracing::info!("Loading pretrained weights...");

        // TODO: Implement weight mapping and loading
        // HuggingFace format weight names:
        //
        // model.embed_tokens.weight
        // model.layers.{i}.self_attn.q_proj.weight
        // model.layers.{i}.self_attn.q_proj.bias
        // model.layers.{i}.self_attn.k_proj.weight
        // model.layers.{i}.self_attn.k_proj.bias
        // model.layers.{i}.self_attn.v_proj.weight
        // model.layers.{i}.self_attn.v_proj.bias
        // model.layers.{i}.self_attn.o_proj.weight
        // model.layers.{i}.input_layernorm.weight
        // model.layers.{i}.post_attention_layernorm.weight
        // model.layers.{i}.mlp.gate_proj.weight
        // model.layers.{i}.mlp.up_proj.weight
        // model.layers.{i}.mlp.down_proj.weight
        // model.norm.weight
        // lm_head.weight

        tracing::warn!("Weight loading not yet implemented - model has random weights");

        Ok(())
    }

    /// Convert safetensors data to Burn tensor
    fn safetensors_to_burn<B: Backend, const D: usize>(
        &self,
        name: &str,
        shape: [usize; D],
        device: &B::Device,
    ) -> Result<Tensor<B, D>> {
        let data = self
            .weight_map
            .get(name)
            .context(format!("Weight not found: {}", name))?;

        // TODO: Parse dtype and convert properly
        // For now, assume f32
        let floats: Vec<f32> = self.bytes_to_f32(data);

        let tensor_data = TensorData::new(floats, shape);
        Ok(Tensor::from_data(tensor_data, device))
    }

    /// Convert bytes to f32 (placeholder - needs proper dtype handling)
    fn bytes_to_f32(&self, bytes: &[u8]) -> Vec<f32> {
        // This is a placeholder - proper implementation needs to:
        // 1. Check the actual dtype from safetensors
        // 2. Convert from that dtype (could be bf16, f16, f32, etc.)
        // 3. Handle endianness correctly

        bytes
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect()
    }
}

/// Load a Qwen2.5 model from a directory containing:
/// - config.json
/// - model.safetensors (or model-*.safetensors shards)
pub fn load_model_from_directory<B: Backend>(
    dir: impl AsRef<Path>,
    device: &B::Device,
) -> Result<Qwen2ForCausalLM<B>> {
    let dir = dir.as_ref();

    // Load config
    let config_path = dir.join("config.json");
    let config = Qwen2Config::from_json(config_path.to_str().unwrap())
        .context("Failed to load config.json")?;

    tracing::info!("Loaded config: {} layers, {} params",
        config.num_hidden_layers,
        estimate_param_count(&config)
    );

    // Create model with random weights
    let mut model = Qwen2ForCausalLM::new(&config, device);

    // Load weights from safetensors
    let model_path = dir.join("model.safetensors");
    if model_path.exists() {
        let loader = WeightLoader::from_file(&model_path)
            .context("Failed to load weights")?;

        loader.load_into_model(&mut model, device)
            .context("Failed to apply weights to model")?;

        tracing::info!("Model loaded successfully");
    } else {
        tracing::warn!("No model.safetensors found - using random weights");
    }

    Ok(model)
}

/// Estimate total parameter count from config
fn estimate_param_count(config: &Qwen2Config) -> String {
    let embedding_params = config.vocab_size * config.hidden_size;

    let layer_params = config.num_hidden_layers * (
        // Attention: Q, K, V, O projections
        4 * config.hidden_size * config.hidden_size +
        // MLP: gate, up, down projections
        config.hidden_size * config.intermediate_size * 2 +
        config.intermediate_size * config.hidden_size +
        // Layer norms (2 per layer)
        2 * config.hidden_size
    );

    let lm_head_params = config.hidden_size * config.vocab_size;
    let total = embedding_params + layer_params + lm_head_params;

    format!("{:.2}B", total as f64 / 1e9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_estimation() {
        let config = Qwen2Config::strand_rust_coder_14b();
        let estimate = estimate_param_count(&config);

        // Should be around 14B parameters
        assert!(estimate.contains("14") || estimate.contains("13") || estimate.contains("15"));
    }
}
