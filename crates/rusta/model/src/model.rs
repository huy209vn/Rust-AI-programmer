//! Qwen2.5 (Strand-Rust-Coder-14B) model architecture in Burn
//!
//! This module contains the complete implementation of the Qwen2.5 transformer
//! architecture, designed for the Strand-Rust-Coder-14B-v1 model.

use burn::{
    config::Config,
    module::Module,
    nn::{
        Embedding, EmbeddingConfig, Linear, LinearConfig, RmsNorm, RmsNormConfig,
        RotaryEncoding, RotaryEncodingConfig, SwiGlu, SwiGluConfig,
    },
    tensor::{activation::softmax, backend::Backend, Bool, Int, Tensor},
};

use crate::cache::AutoregressiveCache;

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for Qwen2.5 model architecture
#[derive(Config, Debug)]
pub struct Qwen2Config {
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub intermediate_size: usize,
    pub num_hidden_layers: usize,
    pub num_attention_heads: usize,
    pub num_key_value_heads: usize,
    pub max_position_embeddings: usize,
    #[config(default = "1e-6")]
    pub rms_norm_eps: f64,
    #[config(default = "1000000.0")]
    pub rope_theta: f64,
    pub hidden_act: String,
    pub bos_token_id: usize,
    pub eos_token_id: usize,
    #[config(default = "false")]
    pub tie_word_embeddings: bool,
}

impl Qwen2Config {
    /// Create configuration for Strand-Rust-Coder-14B-v1
    pub fn strand_rust_coder_14b() -> Self {
        Self {
            vocab_size: 152064,
            hidden_size: 5120,
            intermediate_size: 13824,
            num_hidden_layers: 48,
            num_attention_heads: 40,
            num_key_value_heads: 8, // GQA: 8 key-value heads
            max_position_embeddings: 32768,
            rms_norm_eps: 1e-6,
            rope_theta: 1000000.0,
            hidden_act: "silu".to_string(),
            bos_token_id: 151643,
            eos_token_id: 151645,
            tie_word_embeddings: false,
        }
    }

    /// Initialize the full Qwen2 model for causal language modeling
    pub fn init<B: Backend>(&self, device: &B::Device) -> Qwen2ForCausalLM<B> {
        let model = Qwen2ModelConfig::new(
            self.vocab_size,
            self.num_hidden_layers,
            self.hidden_size,
            self.intermediate_size,
            self.num_attention_heads,
            self.num_key_value_heads,
            self.rms_norm_eps,
            self.rope_theta,
            self.max_position_embeddings,
        )
        .init(device);

        let lm_head = LinearConfig::new(self.hidden_size, self.vocab_size)
            .with_bias(false)
            .init(device);

        Qwen2ForCausalLM { model, lm_head }
    }
}

// ============================================================================
// Model Components
// ============================================================================

/// Configuration for the Qwen2 transformer model
#[derive(Config, Debug)]
pub struct Qwen2ModelConfig {
    pub vocab_size: usize,
    pub num_hidden_layers: usize,
    pub hidden_size: usize,
    pub intermediate_size: usize,
    pub num_attention_heads: usize,
    pub num_key_value_heads: usize,
    pub rms_norm_eps: f64,
    pub rope_theta: f64,
    pub max_position_embeddings: usize,
}

impl Qwen2ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Qwen2Model<B> {
        let embed_tokens = EmbeddingConfig::new(self.vocab_size, self.hidden_size).init(device);

        let layers = (0..self.num_hidden_layers)
            .map(|_| {
                Qwen2DecoderLayerConfig::new(
                    self.hidden_size,
                    self.intermediate_size,
                    self.num_attention_heads,
                    self.num_key_value_heads,
                    self.rms_norm_eps,
                )
                .init(device)
            })
            .collect();

        let norm = RmsNormConfig::new(self.hidden_size)
            .with_epsilon(self.rms_norm_eps)
            .init(device);

        // Initialize RoPE encoding once for all layers
        let head_dim = self.hidden_size / self.num_attention_heads;
        let rope = RotaryEncodingConfig::new(self.max_position_embeddings, head_dim)
            .with_theta(self.rope_theta as f32)
            .init(device);

        Qwen2Model {
            embed_tokens,
            layers,
            norm,
            rope,
        }
    }
}

/// Qwen2 transformer model
#[derive(Module, Debug)]
pub struct Qwen2Model<B: Backend> {
    embed_tokens: Embedding<B>,
    layers: Vec<Qwen2DecoderLayer<B>>,
    norm: RmsNorm<B>,
    rope: RotaryEncoding<B>,
}

impl<B: Backend> Qwen2Model<B> {
    pub fn forward(
        &self,
        input_ids: Tensor<B, 2, Int>,
        cache: &mut Vec<KeyValueCache<B>>,
    ) -> Tensor<B, 3> {
        let mut hidden_states = self.embed_tokens.forward(input_ids);

        for (layer, kv_cache) in self.layers.iter().zip(cache.iter_mut()) {
            hidden_states = layer.forward(hidden_states, kv_cache, &self.rope);
        }

        self.norm.forward(hidden_states)
    }
}

/// Configuration for a Qwen2 decoder layer
#[derive(Config, Debug)]
pub struct Qwen2DecoderLayerConfig {
    pub hidden_size: usize,
    pub intermediate_size: usize,
    pub num_attention_heads: usize,
    pub num_key_value_heads: usize,
    pub rms_norm_eps: f64,
}

impl Qwen2DecoderLayerConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Qwen2DecoderLayer<B> {
        let self_attn = Qwen2AttentionConfig::new(
            self.hidden_size,
            self.num_attention_heads,
            self.num_key_value_heads,
            self.rms_norm_eps,
        )
        .init(device);

        let mlp = Qwen2MLPConfig::new(self.hidden_size, self.intermediate_size).init(device);

        let input_layernorm = RmsNormConfig::new(self.hidden_size)
            .with_epsilon(self.rms_norm_eps)
            .init(device);

        let post_attention_layernorm = RmsNormConfig::new(self.hidden_size)
            .with_epsilon(self.rms_norm_eps)
            .init(device);

        Qwen2DecoderLayer {
            self_attn,
            mlp,
            input_layernorm,
            post_attention_layernorm,
        }
    }
}

/// Qwen2 decoder layer (transformer block)
#[derive(Module, Debug)]
pub struct Qwen2DecoderLayer<B: Backend> {
    self_attn: Qwen2Attention<B>,
    mlp: Qwen2MLP<B>,
    input_layernorm: RmsNorm<B>,
    post_attention_layernorm: RmsNorm<B>,
}

impl<B: Backend> Qwen2DecoderLayer<B> {
    pub fn forward(
        &self,
        hidden_states: Tensor<B, 3>,
        cache: &mut KeyValueCache<B>,
        rope: &RotaryEncoding<B>,
    ) -> Tensor<B, 3> {
        // Self-attention with residual connection
        let residual = hidden_states.clone();
        let hidden_states = self.input_layernorm.forward(hidden_states);
        let hidden_states = self.self_attn.forward(hidden_states, cache, rope);
        let hidden_states = residual + hidden_states;

        // Feed-forward with residual connection
        let residual = hidden_states.clone();
        let hidden_states = self.post_attention_layernorm.forward(hidden_states);
        let hidden_states = self.mlp.forward(hidden_states);
        residual + hidden_states
    }
}

/// Configuration for Qwen2 attention
#[derive(Config, Debug)]
pub struct Qwen2AttentionConfig {
    pub hidden_size: usize,
    pub num_attention_heads: usize,
    pub num_key_value_heads: usize,
    pub rms_norm_eps: f64,
}

impl Qwen2AttentionConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Qwen2Attention<B> {
        let head_dim = self.hidden_size / self.num_attention_heads;

        let q_proj = LinearConfig::new(self.hidden_size, self.num_attention_heads * head_dim)
            .with_bias(true)
            .init(device);

        let k_proj = LinearConfig::new(self.hidden_size, self.num_key_value_heads * head_dim)
            .with_bias(true)
            .init(device);

        let v_proj = LinearConfig::new(self.hidden_size, self.num_key_value_heads * head_dim)
            .with_bias(true)
            .init(device);

        let o_proj = LinearConfig::new(self.num_attention_heads * head_dim, self.hidden_size)
            .with_bias(false)
            .init(device);

        // Q/K normalization for training stability (Qwen2.5-specific)
        let q_norm = RmsNormConfig::new(head_dim)
            .with_epsilon(self.rms_norm_eps)
            .init(device);

        let k_norm = RmsNormConfig::new(head_dim)
            .with_epsilon(self.rms_norm_eps)
            .init(device);

        Qwen2Attention {
            q_proj,
            k_proj,
            v_proj,
            o_proj,
            q_norm,
            k_norm,
            num_heads: self.num_attention_heads,
            num_key_value_heads: self.num_key_value_heads,
            head_dim,
        }
    }
}

/// Qwen2 multi-head attention with Q/K normalization
#[derive(Module, Debug)]
pub struct Qwen2Attention<B: Backend> {
    q_proj: Linear<B>,
    k_proj: Linear<B>,
    v_proj: Linear<B>,
    o_proj: Linear<B>,
    q_norm: RmsNorm<B>,
    k_norm: RmsNorm<B>,
    num_heads: usize,
    num_key_value_heads: usize,
    head_dim: usize,
}

impl<B: Backend> Qwen2Attention<B> {
    pub fn forward(
        &self,
        hidden_states: Tensor<B, 3>,
        cache: &mut KeyValueCache<B>,
        rope: &RotaryEncoding<B>,
    ) -> Tensor<B, 3> {
        let device = hidden_states.device();
        let [batch_size, seq_len, hidden_size] = hidden_states.dims();

        // Project to Q, K, V
        let q = self.q_proj.forward(hidden_states.clone());
        let k = self.k_proj.forward(hidden_states.clone());
        let v = self.v_proj.forward(hidden_states);

        // Reshape to [batch, seq, num_heads, head_dim]
        let q = q.reshape([batch_size, seq_len, self.num_heads, self.head_dim]);
        let k = k.reshape([batch_size, seq_len, self.num_key_value_heads, self.head_dim]);
        let v = v.reshape([batch_size, seq_len, self.num_key_value_heads, self.head_dim]);

        // Apply Q/K normalization (Qwen2.5-specific for training stability)
        let q = self.q_norm.forward(q);
        let k = self.k_norm.forward(k);

        // Swap to [batch, num_heads, seq, head_dim]
        let q = q.swap_dims(1, 2);
        let k = k.swap_dims(1, 2);
        let v = v.swap_dims(1, 2);

        // Apply RoPE
        let cache_seq_len = cache.len();
        let q = rope.apply(q, cache_seq_len);
        let k = rope.apply(k, cache_seq_len);

        // Update KV cache
        let (k, v) = cache.forward(k, v);

        // Repeat K/V heads for GQA (if num_kv_heads < num_heads)
        let k = self.repeat_kv(k);
        let v = self.repeat_kv(v);

        // Compute attention scores
        let mut scores = q
            .matmul(k.swap_dims(2, 3))
            .div_scalar((self.head_dim as f32).sqrt());

        // Apply causal mask for sequences longer than 1
        if seq_len > 1 {
            let cache_seq_len = cache.len();
            let mask = Tensor::<B, 2, Bool>::tril_mask(
                [seq_len, cache_seq_len],
                (cache_seq_len - seq_len) as i64,
                &device,
            );
            scores = scores.mask_fill(mask.unsqueeze::<4>(), f32::NEG_INFINITY);
        }

        let attn_weights = softmax(scores, 3);

        // Apply attention to values
        let attn_output = attn_weights.matmul(v);
        let attn_output = attn_output
            .swap_dims(1, 2)
            .reshape([batch_size, seq_len, hidden_size]);

        self.o_proj.forward(attn_output)
    }

    /// Repeat key/value heads for grouped query attention
    fn repeat_kv(&self, x: Tensor<B, 4>) -> Tensor<B, 4> {
        let n_rep = self.num_heads / self.num_key_value_heads;
        if n_rep == 1 {
            x
        } else {
            let [batch_size, num_kv_heads, seq_len, head_dim] = x.dims();
            x.unsqueeze_dim::<5>(2)
                .expand([batch_size, num_kv_heads, n_rep, seq_len, head_dim])
                .reshape([batch_size, num_kv_heads * n_rep, seq_len, head_dim])
        }
    }
}

/// Configuration for Qwen2 MLP
#[derive(Config, Debug)]
pub struct Qwen2MLPConfig {
    pub hidden_size: usize,
    pub intermediate_size: usize,
}

impl Qwen2MLPConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Qwen2MLP<B> {
        // Use Burn's built-in SwiGLU
        let swiglu = SwiGluConfig::new(self.hidden_size, self.intermediate_size)
            .with_bias(false)
            .init(device);

        let down_proj = LinearConfig::new(self.intermediate_size, self.hidden_size)
            .with_bias(false)
            .init(device);

        Qwen2MLP { swiglu, down_proj }
    }
}

/// Qwen2 MLP with SwiGLU activation
#[derive(Module, Debug)]
pub struct Qwen2MLP<B: Backend> {
    swiglu: SwiGlu<B>,
    down_proj: Linear<B>,
}

impl<B: Backend> Qwen2MLP<B> {
    pub fn forward(&self, hidden_states: Tensor<B, 3>) -> Tensor<B, 3> {
        self.down_proj.forward(self.swiglu.forward(hidden_states))
    }
}

/// Key-value cache for autoregressive generation
pub struct KeyValueCache<B: Backend> {
    key: AutoregressiveCache<B>,
    value: AutoregressiveCache<B>,
}

impl<B: Backend> KeyValueCache<B> {
    /// Create a new key-value cache
    pub fn new(
        max_batch_size: usize,
        num_heads: usize,
        max_seq_len: usize,
        head_dim: usize,
        device: &B::Device,
    ) -> Self {
        Self {
            key: AutoregressiveCache::new(max_batch_size, num_heads, max_seq_len, head_dim, device),
            value: AutoregressiveCache::new(
                max_batch_size,
                num_heads,
                max_seq_len,
                head_dim,
                device,
            ),
        }
    }

    /// Update cache and return full key/value tensors
    pub fn forward(
        &mut self,
        key: Tensor<B, 4>,
        value: Tensor<B, 4>,
    ) -> (Tensor<B, 4>, Tensor<B, 4>) {
        let k = self.key.forward(key);
        let v = self.value.forward(value);
        (k, v)
    }

    /// Get the current cached sequence length
    pub fn len(&self) -> usize {
        self.key.len()
    }

    /// Reset the cache (for new prompts)
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.key.reset();
        self.value.reset();
    }
}

// ============================================================================
// Top-level Model
// ============================================================================

/// Qwen2 model for causal language modeling
#[derive(Module, Debug)]
pub struct Qwen2ForCausalLM<B: Backend> {
    model: Qwen2Model<B>,
    lm_head: Linear<B>,
}

impl<B: Backend> Qwen2ForCausalLM<B> {
    /// Forward pass for next-token prediction
    pub fn forward(
        &self,
        input_ids: Tensor<B, 2, Int>,
        cache: &mut Vec<KeyValueCache<B>>,
    ) -> Tensor<B, 3> {
        let hidden_states = self.model.forward(input_ids, cache);
        self.lm_head.forward(hidden_states)
    }

    /// Initialize KV cache for autoregressive generation
    pub fn init_cache(
        &self,
        config: &Qwen2Config,
        max_batch_size: usize,
        device: &B::Device,
    ) -> Vec<KeyValueCache<B>> {
        let head_dim = config.hidden_size / config.num_attention_heads;
        // Use a practical cache size (2K tokens) instead of max (32K)
        // This avoids CubeCL's single buffer size limit (~1.5GB)
        let practical_max_seq = 2048;

        (0..config.num_hidden_layers)
            .map(|_| {
                KeyValueCache::new(
                    max_batch_size,
                    config.num_key_value_heads,
                    practical_max_seq,
                    head_dim,
                    device,
                )
            })
            .collect()
    }
}
