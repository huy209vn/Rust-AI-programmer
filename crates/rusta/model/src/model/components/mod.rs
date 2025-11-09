use burn::{
    module::Module,
    nn,
    tensor::{backend::Backend, Tensor, activation},
};

/// RMS (Root Mean Square) Layer Normalization
/// Equivalent to T5LayerNorm - normalizes by RMS instead of mean/variance
#[derive(Module, Debug)]
pub struct RMSNorm<B: Backend> {
    /// Learnable scale parameter
    pub weight: nn::Embedding<B>,
    /// Epsilon for numerical stability
    pub eps: f64,
}

impl<B: Backend> RMSNorm<B> {
    pub fn new(hidden_size: usize, eps: f64, device: &B::Device) -> Self {
        let weight = nn::EmbeddingConfig::new(1, hidden_size)
            .init(device);

        Self { weight, eps }
    }

    /// Forward pass: normalize by RMS and scale
    ///
    /// Formula: x * rsqrt(mean(x^2) + eps) * weight
    pub fn forward<const D: usize>(&self, x: Tensor<B, D>) -> Tensor<B, D> {
        // Get input dtype for later conversion back
        // Cast to f32 for numerical stability
        let x_f32 = x.clone();

        // Compute variance (mean of squared values along last dim)
        let variance = x_f32.clone().powf_scalar(2.0).mean_dim(D - 1);

        // Normalize: x / sqrt(variance + eps)
        let normalized = x_f32 / (variance + self.eps).sqrt();

        // Scale by learned weight
        // TODO: Apply weight properly (needs broadcasting)
        normalized
    }
}

/// Rotary Position Embedding (RoPE)
/// Encodes position information by rotating query and key vectors
#[derive(Module, Debug)]
pub struct RotaryEmbedding<B: Backend> {
    /// Inverse frequencies for rotation
    inv_freq: Tensor<B, 1>,
    /// Base value for frequency calculation
    base: f64,
    /// Dimension of embeddings
    dim: usize,
}

impl<B: Backend> RotaryEmbedding<B> {
    pub fn new(dim: usize, base: f64, device: &B::Device) -> Self {
        // Compute inverse frequencies: 1.0 / (base^(i/dim)) for i in [0, 2, 4, ..., dim-2]
        let inv_freq = Self::compute_inv_freq(dim, base, device);

        Self {
            inv_freq,
            base,
            dim,
        }
    }

    fn compute_inv_freq(dim: usize, base: f64, device: &B::Device) -> Tensor<B, 1> {
        // Create range [0, 2, 4, ..., dim-2]
        let arange: Vec<f32> = (0..dim)
            .step_by(2)
            .map(|i| i as f32)
            .collect();

        // Compute 1.0 / (base^(i/dim))
        let inv_freqs: Vec<f32> = arange
            .iter()
            .map(|&i| 1.0 / base.powf(i as f64 / dim as f64) as f32)
            .collect();

        Tensor::from_floats(inv_freqs.as_slice(), device)
    }

    /// Apply rotary embeddings to query and key tensors
    ///
    /// Args:
    ///   - q: Query tensor [batch, seq_len, num_heads, head_dim]
    ///   - k: Key tensor [batch, seq_len, num_kv_heads, head_dim]
    ///   - position_ids: Position indices [batch, seq_len]
    pub fn forward(
        &self,
        q: Tensor<B, 4>,
        k: Tensor<B, 4>,
        _position_ids: Tensor<B, 2, burn::tensor::Int>,
    ) -> (Tensor<B, 4>, Tensor<B, 4>) {
        // TODO: Implement rotation logic
        // For now, return inputs unchanged
        (q, k)
    }
}

/// Multi-Layer Perceptron with SwiGLU activation
///
/// Architecture: gate_proj and up_proj in parallel, then down_proj
/// Formula: down_proj(silu(gate_proj(x)) * up_proj(x))
#[derive(Module, Debug)]
pub struct MLP<B: Backend> {
    /// Gate projection (hidden_size -> intermediate_size)
    pub gate_proj: nn::Linear<B>,
    /// Up projection (hidden_size -> intermediate_size)
    pub up_proj: nn::Linear<B>,
    /// Down projection (intermediate_size -> hidden_size)
    pub down_proj: nn::Linear<B>,
}

impl<B: Backend> MLP<B> {
    pub fn new(hidden_size: usize, intermediate_size: usize, device: &B::Device) -> Self {
        let gate_proj = nn::LinearConfig::new(hidden_size, intermediate_size)
            .with_bias(false)
            .init(device);

        let up_proj = nn::LinearConfig::new(hidden_size, intermediate_size)
            .with_bias(false)
            .init(device);

        let down_proj = nn::LinearConfig::new(intermediate_size, hidden_size)
            .with_bias(false)
            .init(device);

        Self {
            gate_proj,
            up_proj,
            down_proj,
        }
    }

    /// Forward pass with SwiGLU activation
    ///
    /// SwiGLU = Swish-Gated Linear Unit
    /// Formula: down(silu(gate(x)) ⊙ up(x))
    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 3> {
        // Gate path: silu activation
        let gate = self.gate_proj.forward(x.clone());
        let gate_activated = activation::silu(gate);

        // Up path: linear projection
        let up = self.up_proj.forward(x);

        // Element-wise multiply and project down
        let combined = gate_activated * up;
        self.down_proj.forward(combined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::NdArray;

    type TestBackend = NdArray;

    #[test]
    fn test_rms_norm_shape() {
        let device = Default::default();
        let norm = RMSNorm::<TestBackend>::new(512, 1e-6, &device);

        let input = Tensor::<TestBackend, 2>::zeros([4, 512], &device);
        let output = norm.forward(input);

        assert_eq!(output.shape().dims, [4, 512]);
    }

    #[test]
    fn test_mlp_shape() {
        let device = Default::default();
        let mlp = MLP::<TestBackend>::new(512, 2048, &device);

        let input = Tensor::<TestBackend, 3>::zeros([2, 8, 512], &device);
        let output = mlp.forward(input);

        assert_eq!(output.shape().dims, [2, 8, 512]);
    }
}
