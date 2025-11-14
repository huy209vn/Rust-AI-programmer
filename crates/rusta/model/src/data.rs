//! Data loading, tokenization, and batching

use tokenizers::Tokenizer as HFTokenizer;

/// Wrapper around HuggingFace tokenizer for Qwen2
pub struct Qwen2Tokenizer {
    tokenizer: HFTokenizer,
    bos_token_id: u32,
    eos_token_id: u32,
}

impl Qwen2Tokenizer {
    /// Load tokenizer from tokenizer.json file
    ///
    /// # Arguments
    /// * `tokenizer_path` - Path to tokenizer.json file (or directory containing it)
    ///
    /// # Example
    /// ```no_run
    /// use rusta_model::data::Qwen2Tokenizer;
    ///
    /// let tokenizer = Qwen2Tokenizer::from_file(
    ///     "/path/to/Strand-Rust-Coder-14B-v1"
    /// ).unwrap();
    /// ```
    pub fn from_file(tokenizer_path: &str) -> Result<Self, String> {
        let path = std::path::Path::new(tokenizer_path);

        let tokenizer_file = if path.is_dir() {
            path.join("tokenizer.json")
        } else {
            path.to_path_buf()
        };

        if !tokenizer_file.exists() {
            return Err(format!(
                "Tokenizer file not found: {:?}",
                tokenizer_file
            ));
        }

        println!("Loading tokenizer from: {:?}", tokenizer_file);

        let tokenizer = HFTokenizer::from_file(tokenizer_file)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

        // Qwen2 special tokens (from config)
        let bos_token_id = 151643;
        let eos_token_id = 151645;

        Ok(Self {
            tokenizer,
            bos_token_id,
            eos_token_id,
        })
    }

    /// Encode text to token IDs
    ///
    /// # Arguments
    /// * `text` - Input text to encode
    /// * `add_special_tokens` - Whether to add BOS/EOS tokens
    ///
    /// # Returns
    /// Vector of token IDs
    pub fn encode(&self, text: &str, add_special_tokens: bool) -> Result<Vec<u32>, String> {
        let encoding = self
            .tokenizer
            .encode(text, add_special_tokens)
            .map_err(|e| format!("Encoding failed: {}", e))?;

        Ok(encoding.get_ids().to_vec())
    }

    /// Decode token IDs to text
    ///
    /// # Arguments
    /// * `ids` - Token IDs to decode
    /// * `skip_special_tokens` - Whether to skip special tokens in output
    ///
    /// # Returns
    /// Decoded text string
    pub fn decode(&self, ids: &[u32], skip_special_tokens: bool) -> Result<String, String> {
        self.tokenizer
            .decode(ids, skip_special_tokens)
            .map_err(|e| format!("Decoding failed: {}", e))
    }

    /// Get BOS (beginning of sequence) token ID
    pub fn bos_token_id(&self) -> u32 {
        self.bos_token_id
    }

    /// Get EOS (end of sequence) token ID
    pub fn eos_token_id(&self) -> u32 {
        self.eos_token_id
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Only run if model is available
    fn test_tokenizer_roundtrip() {
        let tokenizer = Qwen2Tokenizer::from_file("path/to/model").unwrap();

        let text = "fn main() { println!(\"Hello, world!\"); }";
        let ids = tokenizer.encode(text, false).unwrap();
        let decoded = tokenizer.decode(&ids, false).unwrap();

        assert_eq!(text, decoded);
    }
}
