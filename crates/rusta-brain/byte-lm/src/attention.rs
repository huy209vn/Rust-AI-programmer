use burn::{
    module::Module,
    nn,
    tensor::{backend::Backend, Tensor},
};

use crate::components::RotaryEmbedding;

/// Multi-Head Attention (MHA) for Qwen2.5
///
/// Standard transformer attention with Q, K, V projections
/// All heads have their own key-value pairs (not grouped)
#[derive(Module, Debug)]
pub struct Attention<B: Backend> {
    /// Query projection
    pub q_proj: nn::Linear<B>,
    /// Key projection
    pub k_proj: nn::Linear<B>,
    /// Value projection
    pub v_proj: nn::Linear<B>,
    /// Output projection
    pub o_proj: nn::Linear<B>,

    /// Number of attention heads
    pub num_heads: usize,
    /// Dimension per head
    pub head_dim: usize,
    /// Total hidden size
    pub hidden_size: usize,
}

impl<B: Backend> Attention<B> {
    pub fn new(
        hidden_size: usize,
        num_heads: usize,
        device: &B::Device,
    ) -> Self {
        let head_dim = hidden_size / num_heads;

        let q_proj = nn::LinearConfig::new(hidden_size, hidden_size)
            .with_bias(true)
            .init(device);

        let k_proj = nn::LinearConfig::new(hidden_size, hidden_size)
            .with_bias(true)
            .init(device);

        let v_proj = nn::LinearConfig::new(hidden_size, hidden_size)
            .with_bias(true)
            .init(device);

        let o_proj = nn::LinearConfig::new(hidden_size, hidden_size)
            .with_bias(false)
            .init(device);

        Self {
            q_proj,
            k_proj,
            v_proj,
            o_proj,
            num_heads,
            head_dim,
            hidden_size,
        }
    }

    /// Forward pass for multi-head attention
    ///
    /// Args:
    ///   - hidden_states: Input tensor [batch, seq_len, hidden_size]
    ///   - attention_mask: Optional mask [batch, 1, seq_len, seq_len]
    ///   - position_ids: Position indices [batch, seq_len]
    ///   - rope: Rotary position embedding module
    ///
    /// Returns:
    ///   - Output tensor [batch, seq_len, hidden_size]
    pub fn forward(
        &self,
        hidden_states: Tensor<B, 3>,
        attention_mask: Option<Tensor<B, 4>>,
        position_ids: Tensor<B, 2, burn::tensor::Int>,
        rope: &RotaryEmbedding<B>,
    ) -> Tensor<B, 3> {
        let [batch_size, seq_len, _] = hidden_states.dims();

        // Project to Q, K, V
        let q = self.q_proj.forward(hidden_states.clone());
        let k = self.k_proj.forward(hidden_states.clone());
        let v = self.v_proj.forward(hidden_states);

        // Reshape to separate heads: [batch, seq_len, num_heads, head_dim]
        let q = self.reshape_for_attention(q, batch_size, seq_len);
        let k = self.reshape_for_attention(k, batch_size, seq_len);
        let v = self.reshape_for_attention(v, batch_size, seq_len);

        // Apply rotary embeddings to Q and K
        let (q, k) = rope.forward(q, k, position_ids);

        // Transpose for attention: [batch, num_heads, seq_len, head_dim]
        let q = q.swap_dims(1, 2);
        let k = k.swap_dims(1, 2);
        let v = v.swap_dims(1, 2);

        // Compute attention scores: Q @ K^T / sqrt(head_dim)
        let k_t = k.swap_dims(2, 3);
        let mut scores = q.matmul(k_t);

        // Scale by sqrt(head_dim)
        let scale = 1.0 / (self.head_dim as f32).sqrt();
        scores = scores * scale;

        // Apply attention mask if provided
        if let Some(mask) = attention_mask {
            // Add large negative value to masked positions
            scores = scores + mask * -1e9;
        }

        // Softmax over last dimension
        let attn_weights = burn::tensor::activation::softmax(scores, 3);

        // Apply attention to values: attn_weights @ V
        let context = attn_weights.matmul(v);

        // Transpose back: [batch, seq_len, num_heads, head_dim]
        let context = context.swap_dims(1, 2);

        // Reshape to concatenate heads: [batch, seq_len, hidden_size]
        let context = self.reshape_from_attention(context, batch_size, seq_len);

        // Output projection
        self.o_proj.forward(context)
    }

    /// Reshape tensor for multi-head attention
    /// [batch, seq_len, hidden_size] -> [batch, seq_len, num_heads, head_dim]
    fn reshape_for_attention(
        &self,
        tensor: Tensor<B, 3>,
        batch_size: usize,
        seq_len: usize,
    ) -> Tensor<B, 4> {
        tensor.reshape([batch_size, seq_len, self.num_heads, self.head_dim])
    }

    /// Reshape tensor back from multi-head attention
    /// [batch, seq_len, num_heads, head_dim] -> [batch, seq_len, hidden_size]
    fn reshape_from_attention(
        &self,
        tensor: Tensor<B, 4>,
        batch_size: usize,
        seq_len: usize,
    ) -> Tensor<B, 3> {
        tensor.reshape([batch_size, seq_len, self.hidden_size])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::NdArray;

    type TestBackend = NdArray;

    #[test]
    fn test_attention_shape() {
        let device = Default::default();
        let hidden_size = 512;
        let num_heads = 8;

        let attn = Attention::<TestBackend>::new(hidden_size, num_heads, &device);

        let batch_size = 2;
        let seq_len = 16;

        let hidden_states = Tensor::<TestBackend, 3>::zeros([batch_size, seq_len, hidden_size], &device);
        let position_ids = Tensor::<TestBackend, 2>::zeros([batch_size, seq_len], &device);

        let rope = RotaryEmbedding::<TestBackend>::new(hidden_size / num_heads, 10000.0, &device);

        let output = attn.forward(hidden_states, None, position_ids, &rope);

        assert_eq!(output.shape().dims, [batch_size, seq_len, hidden_size]);
    }
}
