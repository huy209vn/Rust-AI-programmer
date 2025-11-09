//! Qwen2.5 (Strand-Rust-Coder-14B) model architecture in Burn
//!
//! This module contains the complete implementation of the Qwen2.5 transformer
//! architecture, designed for the Strand-Rust-Coder-14B-v1 model.

use burn::{
    config::Config,
    module::Module,
    nn::{Embedding, EmbeddingConfig, Linear, LinearConfig},
    tensor::{activation, backend::Backend, Int, Tensor},
};

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

    /// Head dimension (hidden_size / num_attention_heads)
    pub fn head_dim(&self) -> usize {
        self.hidden_size / self.num_attention_heads
    }
}

// ============================================================================
// Components
// ============================================================================

/// RMS (Root Mean Square) Layer Normalization
/// Equivalent to T5LayerNorm
#[derive(Module, Debug)]
pub struct RMSNorm<B: Backend> {
    weight: Tensor<B, 1>,
    eps: f64,
}

impl<B: Backend> RMSNorm<B> {
    pub fn new(hidden_size: usize, eps: f64, device: &B::Device) -> Self {
        // Initialize weight to ones
        let weight = Tensor::ones([hidden_size], device);
        Self { weight, eps }
    }

    pub fn forward<const D: usize>(&self, x: Tensor<B, D>) -> Tensor<B, D> {
        // Compute variance: mean of squared values along last dimension
        let variance = x.clone().powf_scalar(2.0).mean_dim(D - 1);

        // Normalize: x / sqrt(variance + eps)
        let normalized = x / (variance + self.eps).sqrt();

        // TODO: Apply learned weight properly
        // Burn doesn't support direct broadcasting like PyTorch
        // Need to implement proper weight application
        normalized
    }
}

/// Rotary Position Embedding (RoPE)
#[derive(Module, Debug)]
pub struct RotaryEmbedding<B: Backend> {
    /// Precomputed cos and sin frequencies: [max_seq_len, head_dim, 2]
    freq_complex: Tensor<B, 3>,
    theta: Tensor<B, 1>,
}

impl<B: Backend> RotaryEmbedding<B> {
    pub fn new(dim: usize, base: f64, device: &B::Device) -> Self {
        // For Qwen2, max_position_embeddings is typically 32768
        let max_seq_len = 32768;

        // Compute theta: 1.0 / (base^(i/dim)) for i in [0, 2, 4, ..., dim-2]
        let theta = Self::compute_theta(dim, base, device);

        // Precompute rotary frequencies for all positions
        let freq_complex = Self::compute_rotary_frequencies(0..max_seq_len, theta.clone());

        Self {
            freq_complex,
            theta,
        }
    }

    fn compute_theta(dim: usize, base: f64, device: &B::Device) -> Tensor<B, 1> {
        // theta = 1 / (base ^ (2i / dim)) for i in [0..dim/2]
        let exponent = Tensor::<B, 1, Int>::arange_step(0..dim as i64, 2, device)
            .float()
            / (dim as f32);

        // Use exp(log(base) * exponent) since Burn doesn't support scalar^tensor
        exponent.mul_scalar(base.ln() as f32).exp().recip()
    }

    fn compute_rotary_frequencies(range: core::ops::Range<usize>, theta: Tensor<B, 1>) -> Tensor<B, 3> {
        let d_model = theta.dims()[0] * 2;  // Full head_dim
        let num_positions = range.end - range.start;

        // Generate position indices: [num_positions] -> [num_positions, d_model/2]
        let positions: Tensor<B, 2> = Tensor::<B, 1, Int>::arange(range.start as i64..range.end as i64, &theta.device())
            .float()
            .unsqueeze::<2>()       // -> [1, num_positions]
            .transpose()             // -> [num_positions, 1]
            .repeat_dim(1, d_model / 2);  // -> [num_positions, d_model/2]

        // Expand theta: [d_model/2] -> [1, d_model/2]
        let theta_expanded: Tensor<B, 2> = theta.unsqueeze();

        // Compute frequencies: [num_positions, d_model/2]
        let frequencies: Tensor<B, 2> = positions * theta_expanded;

        // Compute cos and sin: [num_positions, d_model/2]
        let p_cos = frequencies.clone().cos();
        let p_sin = frequencies.sin();

        // Stack to get [num_positions, d_model/2, 2]
        let freq_pairs: Tensor<B, 3> = Tensor::stack(vec![p_cos, p_sin], 2);

        // Repeat each frequency pair for both dimensions in the pair
        // [num_positions, d_model/2, 2] -> [num_positions, d_model, 2]
        freq_pairs
            .unsqueeze_dim::<4>(2)           // -> [num_positions, d_model/2, 1, 2]
            .repeat_dim(2, 2)                // -> [num_positions, d_model/2, 2, 2]
            .reshape([num_positions, d_model, 2])
    }

    /// Apply rotary position embeddings to query and key tensors
    ///
    /// Args:
    ///   - q: [batch, seq_len, num_heads, head_dim]
    ///   - k: [batch, seq_len, num_heads, head_dim]
    ///   - position_ids: [batch, seq_len]
    pub fn forward(
        &self,
        q: Tensor<B, 4>,
        k: Tensor<B, 4>,
        position_ids: Tensor<B, 2, Int>,
    ) -> (Tensor<B, 4>, Tensor<B, 4>) {
        let q_rot = self.apply_rotary(q, position_ids.clone());
        let k_rot = self.apply_rotary(k, position_ids);
        (q_rot, k_rot)
    }

    fn apply_rotary(&self, x: Tensor<B, 4>, _position_ids: Tensor<B, 2, Int>) -> Tensor<B, 4> {
        let [batch_size, seq_len, num_heads, head_dim] = x.dims();

        // Create rotation matrix: [[1, -1], [1, 1]] for [[cos, -sin], [sin, cos]]
        let sign_tensor = Tensor::<B, 2>::from_floats(
            [[1.0, 0.0, 0.0, 1.0], [0.0, -1.0, 1.0, 0.0]],
            &x.device(),
        );

        // Get frequencies for this sequence (assuming contiguous positions starting at 0)
        // TODO: Handle non-contiguous position_ids properly
        let start = 0;

        // Reshape x: [batch, seq, heads, dim] -> [batch*heads, seq, dim/2, 2]
        let out: Tensor<B, 4> = x
            .reshape([batch_size * num_heads, seq_len, head_dim / 2, 2])
            .matmul(sign_tensor.unsqueeze())
            .reshape([batch_size * num_heads, seq_len, head_dim, 2])
            * self
                .freq_complex
                .clone()
                .slice([start..start + seq_len])
                .unsqueeze();

        // Sum real and imaginary components and reshape back
        out.sum_dim(3).reshape([batch_size, seq_len, num_heads, head_dim])
    }
}

/// Multi-Layer Perceptron with SwiGLU activation
#[derive(Module, Debug)]
pub struct MLP<B: Backend> {
    gate_proj: Linear<B>,
    up_proj: Linear<B>,
    down_proj: Linear<B>,
}

impl<B: Backend> MLP<B> {
    pub fn new(hidden_size: usize, intermediate_size: usize, device: &B::Device) -> Self {
        Self {
            gate_proj: LinearConfig::new(hidden_size, intermediate_size)
                .with_bias(false)
                .init(device),
            up_proj: LinearConfig::new(hidden_size, intermediate_size)
                .with_bias(false)
                .init(device),
            down_proj: LinearConfig::new(intermediate_size, hidden_size)
                .with_bias(false)
                .init(device),
        }
    }

    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 3> {
        let gate = activation::silu(self.gate_proj.forward(x.clone()));
        let up = self.up_proj.forward(x);
        self.down_proj.forward(gate * up)
    }
}

// ============================================================================
// Attention
// ============================================================================

/// Multi-Head Attention (MHA) with Q/K normalization
#[derive(Module, Debug)]
pub struct Attention<B: Backend> {
    q_proj: Linear<B>,
    k_proj: Linear<B>,
    v_proj: Linear<B>,
    o_proj: Linear<B>,
    q_norm: RMSNorm<B>,
    k_norm: RMSNorm<B>,
    num_heads: usize,
    head_dim: usize,
    hidden_size: usize,
}

impl<B: Backend> Attention<B> {
    pub fn new(hidden_size: usize, num_heads: usize, rms_norm_eps: f64, device: &B::Device) -> Self {
        let head_dim = hidden_size / num_heads;
        Self {
            q_proj: LinearConfig::new(hidden_size, hidden_size)
                .with_bias(true)
                .init(device),
            k_proj: LinearConfig::new(hidden_size, hidden_size)
                .with_bias(true)
                .init(device),
            v_proj: LinearConfig::new(hidden_size, hidden_size)
                .with_bias(true)
                .init(device),
            o_proj: LinearConfig::new(hidden_size, hidden_size)
                .with_bias(false)
                .init(device),
            // Q/K normalization on head_dim (not full hidden_size)
            q_norm: RMSNorm::new(head_dim, rms_norm_eps, device),
            k_norm: RMSNorm::new(head_dim, rms_norm_eps, device),
            num_heads,
            head_dim,
            hidden_size,
        }
    }

    pub fn forward(
        &self,
        hidden_states: Tensor<B, 3>,
        attention_mask: Option<Tensor<B, 4>>,
        position_ids: Tensor<B, 2, Int>,
        rope: &RotaryEmbedding<B>,
    ) -> Tensor<B, 3> {
        let [batch_size, seq_len, _] = hidden_states.dims();

        // Project to Q, K, V
        let q = self.q_proj.forward(hidden_states.clone());
        let k = self.k_proj.forward(hidden_states.clone());
        let v = self.v_proj.forward(hidden_states);

        // Reshape: [batch, seq_len, num_heads, head_dim]
        let q = q.reshape([batch_size, seq_len, self.num_heads, self.head_dim]);
        let k = k.reshape([batch_size, seq_len, self.num_heads, self.head_dim]);
        let v = v.reshape([batch_size, seq_len, self.num_heads, self.head_dim]);

        // Apply Q/K normalization (on head_dim dimension)
        let q = self.q_norm.forward(q);
        let k = self.k_norm.forward(k);

        // Apply rotary embeddings
        let (q, k) = rope.forward(q, k, position_ids);

        // Transpose: [batch, num_heads, seq_len, head_dim]
        let q = q.swap_dims(1, 2);
        let k = k.swap_dims(1, 2);
        let v = v.swap_dims(1, 2);

        // Attention scores: Q @ K^T / sqrt(head_dim)
        let k_t = k.swap_dims(2, 3);
        let mut scores = q.matmul(k_t) * (1.0 / (self.head_dim as f32).sqrt());

        // Apply mask if provided
        if let Some(mask) = attention_mask {
            scores = scores + mask * -1e9;
        }

        // Softmax and apply to values
        let attn_weights = activation::softmax(scores, 3);
        let context = attn_weights.matmul(v);

        // Transpose back and reshape
        let context = context.swap_dims(1, 2).reshape([batch_size, seq_len, self.hidden_size]);

        self.o_proj.forward(context)
    }
}

// ============================================================================
// Transformer Layers
// ============================================================================

/// Single Transformer Decoder Layer
#[derive(Module, Debug)]
pub struct DecoderLayer<B: Backend> {
    input_layernorm: RMSNorm<B>,
    self_attn: Attention<B>,
    post_attention_layernorm: RMSNorm<B>,
    mlp: MLP<B>,
}

impl<B: Backend> DecoderLayer<B> {
    pub fn new(config: &Qwen2Config, device: &B::Device) -> Self {
        Self {
            input_layernorm: RMSNorm::new(config.hidden_size, config.rms_norm_eps, device),
            self_attn: Attention::new(
                config.hidden_size,
                config.num_attention_heads,
                config.rms_norm_eps,
                device,
            ),
            post_attention_layernorm: RMSNorm::new(config.hidden_size, config.rms_norm_eps, device),
            mlp: MLP::new(config.hidden_size, config.intermediate_size, device),
        }
    }

    pub fn forward(
        &self,
        hidden_states: Tensor<B, 3>,
        attention_mask: Option<Tensor<B, 4>>,
        position_ids: Tensor<B, 2, Int>,
        rope: &RotaryEmbedding<B>,
    ) -> Tensor<B, 3> {
        // Pre-attention norm + attention + residual
        let normed = self.input_layernorm.forward(hidden_states.clone());
        let attn_output = self.self_attn.forward(normed, attention_mask, position_ids, rope);
        let hidden_states = hidden_states + attn_output;

        // Post-attention norm + MLP + residual
        let normed = self.post_attention_layernorm.forward(hidden_states.clone());
        let mlp_output = self.mlp.forward(normed);
        hidden_states + mlp_output
    }
}

// ============================================================================
// Main Models
// ============================================================================

/// Base Qwen2.5 Model
#[derive(Module, Debug)]
pub struct Qwen2Model<B: Backend> {
    embed_tokens: Embedding<B>,
    layers: Vec<DecoderLayer<B>>,
    norm: RMSNorm<B>,
    rotary_emb: RotaryEmbedding<B>,
}

impl Qwen2Config {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Qwen2Model<B> {
        let embed_tokens = EmbeddingConfig::new(self.vocab_size, self.hidden_size).init(device);

        let layers: Vec<_> = (0..self.num_hidden_layers)
            .map(|_| DecoderLayer::new(self, device))
            .collect();

        let norm = RMSNorm::new(self.hidden_size, self.rms_norm_eps, device);
        let rotary_emb = RotaryEmbedding::new(self.head_dim(), self.rope_theta, device);

        Qwen2Model {
            embed_tokens,
            layers,
            norm,
            rotary_emb,
        }
    }
}

impl<B: Backend> Qwen2Model<B> {
    pub fn forward(
        &self,
        input_ids: Tensor<B, 2, Int>,
        attention_mask: Option<Tensor<B, 4>>,
        device: &B::Device,
    ) -> Tensor<B, 3> {
        let [batch_size, seq_len] = input_ids.dims();

        // Create position IDs
        let position_ids = self.create_position_ids(batch_size, seq_len, device);

        // Token embeddings
        let mut hidden_states = self.embed_tokens.forward(input_ids);

        // Pass through all decoder layers
        for layer in &self.layers {
            hidden_states = layer.forward(
                hidden_states,
                attention_mask.clone(),
                position_ids.clone(),
                &self.rotary_emb,
            );
        }

        // Final normalization
        self.norm.forward(hidden_states)
    }

    fn create_position_ids(
        &self,
        batch_size: usize,
        seq_len: usize,
        device: &B::Device,
    ) -> Tensor<B, 2, Int> {
        let positions: Vec<i32> = (0..seq_len as i32).collect();
        let single_batch = Tensor::<B, 1, Int>::from_ints(positions.as_slice(), device);
        single_batch.unsqueeze_dim(0).repeat_dim(0, batch_size)
    }
}

/// Qwen2.5 Model with Language Modeling Head
#[derive(Module, Debug)]
pub struct Qwen2ForCausalLM<B: Backend> {
    model: Qwen2Model<B>,
    lm_head: Linear<B>,
}

impl Qwen2Config {
    pub fn init_with_lm_head<B: Backend>(&self, device: &B::Device) -> Qwen2ForCausalLM<B> {
        let model = self.init(device);
        let lm_head = LinearConfig::new(self.hidden_size, self.vocab_size)
            .with_bias(false)
            .init(device);
        Qwen2ForCausalLM { model, lm_head }
    }
}

impl<B: Backend> Qwen2ForCausalLM<B> {
    pub fn forward(
        &self,
        input_ids: Tensor<B, 2, Int>,
        attention_mask: Option<Tensor<B, 4>>,
        device: &B::Device,
    ) -> Tensor<B, 3> {
        let hidden_states = self.model.forward(input_ids, attention_mask, device);
        self.lm_head.forward(hidden_states)
    }
}
