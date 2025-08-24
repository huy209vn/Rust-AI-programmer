//! model-core — Burn decoder-only transformer core (skeleton)
pub struct ModelConfig {
    pub d_model: usize,
    pub n_layers: usize,
    pub n_heads: usize,
    pub vocab: usize, // bytes = 256
}
impl Default for ModelConfig {
    fn default() -> Self {
        Self { d_model: 512, n_layers: 8, n_heads: 8, vocab: 256 }
    }
}
pub fn ping() -> &'static str { "model-core:ok" }
