# Rusta Model - Strand-Rust-Coder-14B-v1 in Burn

This crate provides a complete implementation of the **Strand-Rust-Coder-14B-v1** model using the [Burn](https://github.com/tracel-ai/burn) deep learning framework.

## Model Architecture

- **Base Model**: Qwen2.5-14B
- **Architecture**: Transformer decoder with Grouped Query Attention (GQA)
- **Parameters**: 14B
- **Context Length**: 32,768 tokens
- **Vocabulary Size**: 152,064
- **Hidden Size**: 5,120
- **Layers**: 48
- **Attention Heads**: 40 (8 key-value heads for GQA)
- **Activation**: SwiGLU

## Features

- ✅ Complete Qwen2.5 architecture implementation in Burn
- ✅ Safetensors weight loading (sharded and single file)
- ✅ HuggingFace tokenizer integration
- ✅ KV-cache for efficient autoregressive generation
- ✅ RoPE (Rotary Position Embeddings)
- ✅ Q/K normalization for training stability
- ✅ Grouped Query Attention (GQA)

## Installation

### 1. Download the Model

Download the Strand-Rust-Coder-14B-v1 model from HuggingFace:

```bash
# Using HuggingFace CLI (recommended)
pip install huggingface-hub
huggingface-cli download Fortytwo-Network/Strand-Rust-Coder-14B-v1

# Or using git
git lfs install
git clone https://huggingface.co/Fortytwo-Network/Strand-Rust-Coder-14B-v1
```

The model will be downloaded to:
- **Linux/Mac**: `~/.cache/huggingface/hub/models--Fortytwo-Network--Strand-Rust-Coder-14B-v1/snapshots/<hash>/`
- **Windows**: `C:\Users\<username>\.cache\huggingface\hub\models--Fortytwo-Network--Strand-Rust-Coder-14B-v1\snapshots\<hash>\`

### 2. Add Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
rusta-model = { path = "path/to/rusta/model" }
burn = "0.19.0"
```

## Usage

### Running the Example

```bash
# On Windows (adjust path to your model location)
cargo run --example text_generation --release -- \
    --model-path "C:\Users\PC\.cache\huggingface\hub\models--Fortytwo-Network--Strand-Rust-Coder-14B-v1\snapshots\0b9a97c5ab89f9780c95356cc2ea121eb434372e" \
    --prompt "fn fibonacci(n: u32) -> u32 {" \
    --max-tokens 200 \
    --temperature 0.7

# On Linux/Mac
cargo run --example text_generation --release -- \
    --model-path ~/.cache/huggingface/hub/models--Fortytwo-Network--Strand-Rust-Coder-14B-v1/snapshots/<hash> \
    --prompt "fn fibonacci(n: u32) -> u32 {" \
    --max-tokens 200 \
    --temperature 0.7
```

### Programmatic Usage

```rust
use burn::tensor::{backend::Backend, Int, Tensor};
use rusta_model::{load_model, generate, Qwen2Config, Qwen2Tokenizer};

// Choose backend (CPU or GPU)
type MyBackend = burn::backend::NdArray<f32>;
// For CUDA: type MyBackend = burn::backend::LibTorch<f32>;

fn main() {
    let device = Default::default();
    let model_path = "/path/to/Strand-Rust-Coder-14B-v1";

    // Load tokenizer
    let tokenizer = Qwen2Tokenizer::from_file(model_path)
        .expect("Failed to load tokenizer");

    // Load model
    let model = load_model::<MyBackend>(model_path, &device)
        .expect("Failed to load model");

    // Encode prompt
    let prompt = "fn main() {";
    let input_ids = tokenizer.encode(prompt, false).unwrap();
    let input_tensor: Tensor<MyBackend, 2, Int> = Tensor::from_ints(
        input_ids.iter().map(|&id| id as i64).collect::<Vec<_>>().as_slice(),
        &device,
    ).reshape([1, input_ids.len()]);

    // Generate
    let config = Qwen2Config::strand_rust_coder_14b();
    let output = generate(&model, &config, input_tensor, 100, 0.7, &device);

    // Decode
    let output_ids: Vec<u32> = output
        .into_data()
        .to_vec::<i64>()
        .unwrap()
        .iter()
        .map(|&id| id as u32)
        .collect();
    let text = tokenizer.decode(&output_ids, true).unwrap();
    println!("{}", text);
}
```

## Performance Tips

### Backend Selection

- **CPU (NdArray)**: Good for testing, slower inference
- **GPU (LibTorch with CUDA)**: Recommended for production, requires CUDA and libtorch

To use LibTorch backend:

```toml
[dependencies]
burn = { version = "0.19.0", features = ["libtorch"] }
```

```rust
type MyBackend = burn::backend::LibTorch<f32>;
```

### Memory Requirements

The model requires approximately:
- **FP32**: ~56 GB RAM/VRAM
- **FP16**: ~28 GB RAM/VRAM (recommended minimum)

For CPU inference, ensure you have sufficient RAM. For GPU inference, use a GPU with at least 24GB VRAM (consider model quantization for smaller GPUs).

### Compilation

Always use `--release` mode for inference:

```bash
cargo run --release --example text_generation
```

Release mode is 10-100x faster than debug mode for deep learning inference.

## Model Files

Required files in the model directory:

- `model.safetensors.index.json` - Index for sharded weights
- `model-00001-of-00006.safetensors` through `model-00006-of-00006.safetensors` - Model weights
- `tokenizer.json` - Tokenizer vocabulary and merges
- `config.json` - Model configuration (optional, hardcoded in our implementation)

## Architecture Details

### Qwen2.5 Improvements

This implementation includes Qwen2.5-specific features:

1. **Q/K Normalization**: RMSNorm applied to query and key vectors for improved training stability
2. **Grouped Query Attention**: 40 query heads, 8 key-value heads (5:1 ratio) for efficiency
3. **RoPE**: Rotary position embeddings with theta=1,000,000 for long context
4. **SwiGLU**: Swish-gated linear unit activation in MLP

### File Structure

```
src/
├── lib.rs         - Public exports
├── model.rs       - Model architecture (Qwen2.5)
├── cache.rs       - KV-cache for autoregressive generation
├── data.rs        - Tokenizer wrapper
├── inference.rs   - Weight loading and generation
└── training.rs    - Training utilities (TODO)

examples/
└── text_generation.rs - Complete inference example
```

## Troubleshooting

### Tokenizer Loading Fails

**Error**: `Tokenizer file not found`

**Solution**: Ensure `tokenizer.json` exists in the model directory. Download it from HuggingFace if missing.

### Weight Loading Fails

**Error**: `Failed to load safetensors`

**Solution**:
- Verify all 6 safetensors files are present
- Check that `model.safetensors.index.json` exists
- Ensure files are not corrupted (re-download if necessary)

### Out of Memory

**Error**: OOM during model loading or inference

**Solution**:
- Close other applications
- Use a machine with more RAM
- Consider quantization (future feature)
- Use GPU if available

### Slow Inference

**Problem**: Generation is very slow

**Solution**:
- Use `--release` mode
- Use GPU backend (LibTorch with CUDA)
- Reduce `max_new_tokens`
- Use batch size of 1 for single prompts

## Future Enhancements

- [ ] Quantization support (INT8, INT4)
- [ ] Flash Attention for faster inference
- [ ] Batched generation
- [ ] Top-p and top-k sampling
- [ ] Streaming token generation
- [ ] Fine-tuning support
- [ ] WGPU backend for cross-platform GPU

## License

This implementation follows the Rusta project license. The Strand-Rust-Coder-14B-v1 model weights are provided by Fortytwo-Network under their license terms.

## Citation

```bibtex
@software{strand-rust-coder-14b-v1,
  title = {Strand-Rust-Coder-14B-v1},
  author = {Fortytwo-Network},
  url = {https://huggingface.co/Fortytwo-Network/Strand-Rust-Coder-14B-v1},
  year = {2024}
}
```

## Contributing

Contributions welcome! Please ensure:
- Code follows Rust idioms
- All tests pass: `cargo test`
- Clippy is happy: `cargo clippy`
- Code is formatted: `cargo fmt`

## Support

For issues specific to this implementation, open an issue in the Rusta repository.
For model-specific questions, see the [Strand-Rust-Coder-14B-v1 model card](https://huggingface.co/Fortytwo-Network/Strand-Rust-Coder-14B-v1).
