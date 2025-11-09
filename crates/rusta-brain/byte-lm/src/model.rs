use burn::{
    module::Module,
    nn,
    tensor::{backend::Backend, Tensor},
};

use crate::{
    attention::Attention,
    components::{MLP, RMSNorm, RotaryEmbedding},
    config::Qwen2Config,
};

/// Single Transformer Decoder Layer
///
/// Architecture:
/// 1. Input LayerNorm
/// 2. Self-Attention
/// 3. Residual connection
/// 4. Post-attention LayerNorm
/// 5. MLP (feedforward)
/// 6. Residual connection
#[derive(Module, Debug)]
pub struct DecoderLayer<B: Backend> {
    /// Pre-attention layer normalization
    pub input_layernorm: RMSNorm<B>,
    /// Self-attention module
    pub self_attn: Attention<B>,
    /// Post-attention layer normalization
    pub post_attention_layernorm: RMSNorm<B>,
    /// MLP feedforward network
    pub mlp: MLP<B>,
}

impl<B: Backend> DecoderLayer<B> {
    pub fn new(config: &Qwen2Config, device: &B::Device) -> Self {
        let input_layernorm = RMSNorm::new(
            config.hidden_size,
            config.rms_norm_eps,
            device,
        );

        let self_attn = Attention::new(
            config.hidden_size,
            config.num_attention_heads,
            device,
        );

        let post_attention_layernorm = RMSNorm::new(
            config.hidden_size,
            config.rms_norm_eps,
            device,
        );

        let mlp = MLP::new(
            config.hidden_size,
            config.intermediate_size,
            device,
        );

        Self {
            input_layernorm,
            self_attn,
            post_attention_layernorm,
            mlp,
        }
    }

    /// Forward pass through decoder layer
    ///
    /// Args:
    ///   - hidden_states: Input [batch, seq_len, hidden_size]
    ///   - attention_mask: Optional mask [batch, 1, seq_len, seq_len]
    ///   - position_ids: Position indices [batch, seq_len]
    ///   - rope: Rotary embedding module
    pub fn forward(
        &self,
        hidden_states: Tensor<B, 3>,
        attention_mask: Option<Tensor<B, 4>>,
        position_ids: Tensor<B, 2, burn::tensor::Int>,
        rope: &RotaryEmbedding<B>,
    ) -> Tensor<B, 3> {
        // Pre-attention norm
        let normed = self.input_layernorm.forward(hidden_states.clone());

        // Self-attention
        let attn_output = self.self_attn.forward(
            normed,
            attention_mask,
            position_ids,
            rope,
        );

        // First residual connection
        let hidden_states = hidden_states + attn_output;

        // Post-attention norm
        let normed = self.post_attention_layernorm.forward(hidden_states.clone());

        // MLP
        let mlp_output = self.mlp.forward(normed);

        // Second residual connection
        hidden_states + mlp_output
    }
}

/// Base Qwen2.5 Model (without language modeling head)
///
/// Architecture:
/// 1. Token embeddings
/// 2. Stack of decoder layers
/// 3. Final layer normalization
#[derive(Module, Debug)]
pub struct Qwen2Model<B: Backend> {
    /// Token embedding layer
    pub embed_tokens: nn::Embedding<B>,
    /// Stack of transformer decoder layers
    pub layers: Vec<DecoderLayer<B>>,
    /// Final layer normalization
    pub norm: RMSNorm<B>,
    /// Rotary position embedding (shared across layers)
    pub rotary_emb: RotaryEmbedding<B>,
}

impl<B: Backend> Qwen2Model<B> {
    pub fn new(config: &Qwen2Config, device: &B::Device) -> Self {
        // Token embeddings
        let embed_tokens = nn::EmbeddingConfig::new(config.vocab_size, config.hidden_size)
            .init(device);

        // Build decoder layers
        let layers: Vec<_> = (0..config.num_hidden_layers)
            .map(|_| DecoderLayer::new(config, device))
            .collect();

        // Final norm
        let norm = RMSNorm::new(config.hidden_size, config.rms_norm_eps, device);

        // Rotary embeddings (shared across all layers)
        let rotary_emb = RotaryEmbedding::new(
            config.head_dim(),
            config.rope_theta,
            device,
        );

        Self {
            embed_tokens,
            layers,
            norm,
            rotary_emb,
        }
    }

    /// Forward pass through the model
    ///
    /// Args:
    ///   - input_ids: Token IDs [batch, seq_len]
    ///   - attention_mask: Optional causal mask
    ///   - device: Device for creating position IDs
    ///
    /// Returns:
    ///   - Hidden states [batch, seq_len, hidden_size]
    pub fn forward(
        &self,
        input_ids: Tensor<B, 2, burn::tensor::Int>,
        attention_mask: Option<Tensor<B, 4>>,
        device: &B::Device,
    ) -> Tensor<B, 3> {
        let [batch_size, seq_len] = input_ids.dims();

        // Create position IDs [0, 1, 2, ..., seq_len-1]
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

    /// Create position IDs tensor: [0, 1, 2, ..., seq_len-1] for each batch
    fn create_position_ids(
        &self,
        batch_size: usize,
        seq_len: usize,
        device: &B::Device,
    ) -> Tensor<B, 2, burn::tensor::Int> {
        let positions: Vec<i32> = (0..seq_len as i32).collect();
        let single_batch = Tensor::<B, 1, burn::tensor::Int>::from_ints(
            positions.as_slice(),
            device,
        );

        // Repeat for batch
        single_batch.unsqueeze_dim(0).repeat_dim(0, batch_size)
    }
}

/// Qwen2.5 Model with Language Modeling Head
///
/// Adds a linear projection from hidden states to vocabulary logits
#[derive(Module, Debug)]
pub struct Qwen2ForCausalLM<B: Backend> {
    /// Base transformer model
    pub model: Qwen2Model<B>,
    /// Language modeling head (hidden_size -> vocab_size)
    pub lm_head: nn::Linear<B>,
}

impl<B: Backend> Qwen2ForCausalLM<B> {
    pub fn new(config: &Qwen2Config, device: &B::Device) -> Self {
        let model = Qwen2Model::new(config, device);

        let lm_head = nn::LinearConfig::new(config.hidden_size, config.vocab_size)
            .with_bias(false)
            .init(device);

        Self { model, lm_head }
    }

    /// Forward pass for language modeling
    ///
    /// Args:
    ///   - input_ids: Token IDs [batch, seq_len]
    ///   - attention_mask: Optional causal mask
    ///   - device: Device for computation
    ///
    /// Returns:
    ///   - Logits over vocabulary [batch, seq_len, vocab_size]
    pub fn forward(
        &self,
        input_ids: Tensor<B, 2, burn::tensor::Int>,
        attention_mask: Option<Tensor<B, 4>>,
        device: &B::Device,
    ) -> Tensor<B, 3> {
        // Get hidden states from base model
        let hidden_states = self.model.forward(input_ids, attention_mask, device);

        // Project to vocabulary logits
        self.lm_head.forward(hidden_states)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::NdArray;

    type TestBackend = NdArray;

    #[test]
    fn test_decoder_layer_shape() {
        let device = Default::default();
        let config = Qwen2Config {
            vocab_size: 1000,
            hidden_size: 512,
            intermediate_size: 2048,
            num_hidden_layers: 6,
            num_attention_heads: 8,
            num_key_value_heads: 8,
            max_position_embeddings: 2048,
            rms_norm_eps: 1e-6,
            rope_theta: 10000.0,
            hidden_act: "silu".to_string(),
            bos_token_id: 1,
            eos_token_id: 2,
            tie_word_embeddings: false,
        };

        let layer = DecoderLayer::<TestBackend>::new(&config, &device);
        let rope = RotaryEmbedding::<TestBackend>::new(64, 10000.0, &device);

        let batch_size = 2;
        let seq_len = 16;
        let hidden_states = Tensor::<TestBackend, 3>::zeros([batch_size, seq_len, config.hidden_size], &device);
        let position_ids = Tensor::<TestBackend, 2>::zeros([batch_size, seq_len], &device);

        let output = layer.forward(hidden_states, None, position_ids, &rope);

        assert_eq!(output.shape().dims, [batch_size, seq_len, config.hidden_size]);
    }

    #[test]
    fn test_qwen2_model_shape() {
        let device = Default::default();
        let config = Qwen2Config {
            vocab_size: 1000,
            hidden_size: 256,
            intermediate_size: 1024,
            num_hidden_layers: 4,
            num_attention_heads: 4,
            num_key_value_heads: 4,
            max_position_embeddings: 512,
            rms_norm_eps: 1e-6,
            rope_theta: 10000.0,
            hidden_act: "silu".to_string(),
            bos_token_id: 1,
            eos_token_id: 2,
            tie_word_embeddings: false,
        };

        let model = Qwen2ForCausalLM::<TestBackend>::new(&config, &device);

        let batch_size = 2;
        let seq_len = 8;
        let input_ids = Tensor::<TestBackend, 2, burn::tensor::Int>::zeros([batch_size, seq_len], &device);

        let logits = model.forward(input_ids, None);

        assert_eq!(logits.shape().dims, [batch_size, seq_len, config.vocab_size]);
    }
}
