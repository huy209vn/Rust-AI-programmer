//! Key-value caching for autoregressive generation

use burn::tensor::{backend::Backend, Tensor};

/// Autoregressive cache for storing key or value tensors during generation
pub struct AutoregressiveCache<B: Backend> {
    cache: Tensor<B, 4>,
    current_len: usize,
}

impl<B: Backend> AutoregressiveCache<B> {
    /// Create a new autoregressive cache
    ///
    /// # Arguments
    /// * `max_batch_size` - Maximum batch size
    /// * `num_heads` - Number of attention heads
    /// * `max_seq_len` - Maximum sequence length
    /// * `head_dim` - Dimension of each attention head
    /// * `device` - Device to allocate the cache on
    pub fn new(
        max_batch_size: usize,
        num_heads: usize,
        max_seq_len: usize,
        head_dim: usize,
        device: &B::Device,
    ) -> Self {
        let cache = Tensor::zeros(
            [max_batch_size, num_heads, max_seq_len, head_dim],
            device,
        );

        Self {
            cache,
            current_len: 0,
        }
    }

    /// Append new key/value tensor and return the full cached sequence
    ///
    /// # Arguments
    /// * `new_data` - New key or value tensor with shape [batch, num_heads, seq_len, head_dim]
    ///
    /// # Returns
    /// Full cached tensor with shape [batch, num_heads, current_len + seq_len, head_dim]
    pub fn forward(&mut self, new_data: Tensor<B, 4>) -> Tensor<B, 4> {
        let [batch_size, num_heads, new_seq_len, head_dim] = new_data.dims();
        let [max_batch, max_heads, _max_seq, max_head_dim] = self.cache.dims();

        // Update the cache with new data
        let end_pos = self.current_len + new_seq_len;
        self.cache = self
            .cache
            .clone()
            .slice_assign(
                [0..batch_size, 0..num_heads, self.current_len..end_pos, 0..head_dim],
                new_data.clone(),
            );

        // Update current length
        self.current_len = end_pos;

        // Return the active portion of the cache
        self.cache
            .clone()
            .slice([0..batch_size, 0..num_heads, 0..end_pos, 0..head_dim])
    }

    /// Get the current cached sequence length
    pub fn len(&self) -> usize {
        self.current_len
    }

    /// Reset the cache (for new prompts)
    pub fn reset(&mut self) {
        self.current_len = 0;
    }
}
