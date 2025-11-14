# 🚀 Strand-Rust-Coder-14B-v1 Quick Start Guide

## What We've Built

You now have a complete implementation of the **Strand-Rust-Coder-14B-v1** model in Rust using the Burn framework! This is a 14B parameter transformer model specialized for Rust code generation.

## 📁 Your Model Location

Your downloaded model is at:
```
C:\Users\PC\.cache\huggingface\hub\models--Fortytwo-Network--Strand-Rust-Coder-14B-v1\snapshots\0b9a97c5ab89f9780c95356cc2ea121eb434372e
```

## 🏃 Quick Test Run

### Prerequisites: CUDA Toolkit

**IMPORTANT**: This model uses native CUDA GPU acceleration via CubeCL (pure Rust!).

#### What You Need:
- **CUDA-capable GPU** (NVIDIA, 16GB+ VRAM recommended)
- **CUDA Toolkit** 11.8+ or 12.x
- **That's it!** No LibTorch, no PyTorch - pure Rust 🦀

#### Installing CUDA Toolkit:

**Windows:**
1. Download from: https://developer.nvidia.com/cuda-downloads
2. Install CUDA Toolkit (includes nvcc compiler and libraries)
3. Verify: `nvcc --version` should work

**Linux:**
```bash
# Ubuntu/Debian
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-keyring_1.1-1_all.deb
sudo dpkg -i cuda-keyring_1.1-1_all.deb
sudo apt-get update
sudo apt-get install cuda-toolkit-12-3

# Or check your distro's CUDA installation guide
```

### Run the Example

```bash
# Make sure you're in the project root
cd /path/to/Rust-AI-programmer

# Run the text generation example (native CUDA via CubeCL!)
cargo run --example text_generation --release -- \
    --model-path "C:\Users\PC\.cache\huggingface\hub\models--Fortytwo-Network--Strand-Rust-Coder-14B-v1\snapshots\0b9a97c5ab89f9780c95356cc2ea121eb434372e" \
    --prompt "fn fibonacci(n: u32) -> u32 {" \
    --max-tokens 150 \
    --temperature 0.7
```

### Option 2: Try Different Prompts

```bash
# Generate a hash map implementation
cargo run --example text_generation --release -- \
    --model-path "YOUR_MODEL_PATH" \
    --prompt "use std::collections::HashMap;\n\nfn count_words(text: &str) -> HashMap<String, usize> {" \
    --max-tokens 200

# Generate error handling code
cargo run --example text_generation --release -- \
    --model-path "YOUR_MODEL_PATH" \
    --prompt "fn read_config(path: &Path) -> Result<Config, ConfigError> {" \
    --max-tokens 150
```

## ⚙️ Parameters Explained

- `--model-path`: Path to your downloaded model directory
- `--prompt`: The starting text (Rust code) to continue
- `--max-tokens`: How many new tokens to generate (default: 100)
- `--temperature`: Randomness (0.1 = deterministic, 1.0 = creative, default: 0.7)

## 🔧 What's Implemented

### Core Architecture ✅
- ✅ **Full Qwen2.5 transformer** (48 layers, 5120 hidden size)
- ✅ **Grouped Query Attention** (GQA) for efficiency
- ✅ **RoPE** (Rotary Position Embeddings)
- ✅ **Q/K Normalization** for stability
- ✅ **SwiGLU activation** in MLP
- ✅ **KV-cache** for fast autoregressive generation

### Weight Loading ✅
- ✅ **Sharded safetensors** support (6 files)
- ✅ **Automatic weight mapping** from PyTorch format

### Tokenization ✅
- ✅ **HuggingFace tokenizer** integration
- ✅ **Qwen2 vocabulary** (152,064 tokens)
- ✅ **Encoding/decoding** with special tokens

## 📊 Performance Tips

### First Run
The **first run will be slow** because:
1. Rust needs to compile in release mode (~5-10 min)
2. Model weights need to be loaded (~1-2 min for 14B params)
3. Burn framework initializes

### Subsequent Runs
After compilation, only model loading takes time.

### Expected Speed
- **GPU (Native CUDA/CubeCL)**: ~20-80+ tokens/second (depends on your GPU)
- This implementation uses pure Rust CUDA via CubeCL - no LibTorch!
- RTX 4090, RTX 3090, A100, etc. should all work great

### Memory Requirements
- **GPU VRAM**: 16GB minimum, 24GB+ recommended
- **System RAM**: 16GB+ recommended
- The model will load into GPU memory for fastest inference

## 🎯 Example Output

**Prompt:**
```rust
fn fibonacci(n: u32) -> u32 {
```

**Generated:**
```rust
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

## 🐛 Troubleshooting

### "Failed to load tokenizer"
**Problem**: `tokenizer.json` not found

**Solution**: Check that your model directory contains `tokenizer.json`. You can verify with:
```bash
ls "C:\Users\PC\.cache\huggingface\hub\models--Fortytwo-Network--Strand-Rust-Coder-14B-v1\snapshots\0b9a97c5ab89f9780c95356cc2ea121eb434372e"
```

Should show:
- `tokenizer.json`
- `model.safetensors.index.json`
- `model-00001-of-00006.safetensors` through `model-00006-of-00006.safetensors`

### "Failed to load safetensors"
**Problem**: Weight files corrupted or missing

**Solution**: Re-download the model:
```bash
huggingface-cli download Fortytwo-Network/Strand-Rust-Coder-14B-v1 --resume-download
```

### Out of Memory
**Problem**: System runs out of RAM during loading/inference

**Solutions**:
1. Close other applications
2. Use a machine with more RAM
3. Consider using quantization (future feature)

### Very Slow Generation
**Problem**: Generating 1-2 tokens takes minutes

**Solutions**:
1. Make sure you used `--release` flag (CRITICAL for performance!)
2. Verify CUDA is working: `nvidia-smi` should show your GPU
3. Check CUDA Toolkit is installed: `nvcc --version`
4. Wait for first token (prompt processing takes longer than subsequent tokens)
5. Make sure you're using the CUDA backend (should see "Using device: CUDA" in output)

## 📚 Next Steps

### 1. Integrate into Your Project
```rust
use burn::tensor::{backend::Backend, Int, Tensor};
use rusta_model::{load_model, generate, Qwen2Config, Qwen2Tokenizer};

// See crates/rusta/model/README.md for full API docs
```

### 2. Fine-tune the Model
See `training.rs` (coming soon) for fine-tuning on your own Rust code

### 3. Build a Code Assistant
Integrate with your editor/IDE for real-time code completion

## 🔥 Example Use Cases

1. **Code Completion**: Continue partially written functions
2. **Test Generation**: Generate unit tests from function signatures
3. **Documentation**: Generate doc comments from code
4. **Refactoring**: Suggest idiomatic Rust rewrites
5. **Bug Fixing**: Complete error handling patterns

## 📖 Full Documentation

See `crates/rusta/model/README.md` for:
- Complete API reference
- Architecture details
- Programming examples
- Contribution guidelines

## 🎉 You're Ready!

Run your first generation and see Rust code come to life! 🦀✨

```bash
cargo run --example text_generation --release -- \
    --model-path "YOUR_MODEL_PATH" \
    --prompt "// Write a function to" \
    --max-tokens 100
```

---

**Questions or Issues?**
- Check the README: `crates/rusta/model/README.md`
- Open an issue in the repository
- Review the code in `crates/rusta/model/src/`

Happy coding! 🚀
