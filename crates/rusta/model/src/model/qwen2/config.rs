use serde::{Deserialize, Serialize};

/// Configuration for Qwen2.5 model architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qwen2Config {
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub intermediate_size: usize,
    pub num_hidden_layers: usize,
    pub num_attention_heads: usize,
    pub num_key_value_heads: usize,
    pub max_position_embeddings: usize,
    pub rms_norm_eps: f64,
    pub rope_theta: f64,
    pub hidden_act: String,
    pub bos_token_id: usize,
    pub eos_token_id: usize,
    pub tie_word_embeddings: bool,
}

impl Qwen2Config {
    /// Create configuration for Strand-Rust-Coder-14B-v1
    /// Modified to use full MHA (not GQA) for simpler implementation
    pub fn strand_rust_coder_14b() -> Self {
        Self {
            vocab_size: 152064,
            hidden_size: 5120,
            intermediate_size: 13824,
            num_hidden_layers: 48,
            num_attention_heads: 40,
            num_key_value_heads: 40, // Full MHA (original model uses 8 for GQA)
            max_position_embeddings: 32768,
            rms_norm_eps: 1e-6,
            rope_theta: 1_000_000.0,
            hidden_act: "silu".to_string(),
            bos_token_id: 151643,
            eos_token_id: 151645,
            tie_word_embeddings: false,
        }
    }

    /// Load config from JSON file
    pub fn from_json(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Head dimension (hidden_size / num_attention_heads)
    pub fn head_dim(&self) -> usize {
        self.hidden_size / self.num_attention_heads
    }

    /// Number of query groups (for grouped query attention)
    pub fn num_key_value_groups(&self) -> usize {
        self.num_attention_heads / self.num_key_value_heads
    }
}
