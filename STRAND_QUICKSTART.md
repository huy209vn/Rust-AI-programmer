# 🚀 Strand-Rust-Coder-14B-v1 Quick Start Guide

## What We've Built

You now have a complete implementation of the **Strand-Rust-Coder-14B-v1** model in Rust using the Burn framework! This is a 14B parameter transformer model specialized for Rust code generation.

## 📁 Your Model Location

Your downloaded model is at:
```
C:\Users\PC\.cache\huggingface\hub\models--Fortytwo-Network--Strand-Rust-Coder-14B-v1\snapshots\0b9a97c5ab89f9780c95356cc2ea121eb434372e
```

## 🏃 Quick Test Run

### Prerequisites: LibTorch Installation

**IMPORTANT**: This model uses CUDA GPU acceleration via LibTorch. You need to install LibTorch first:

#### Windows:
1. Download LibTorch from https://pytorch.org/get-started/locally/
   - Select: PyTorch Build = Stable, Your OS = Windows, Package = LibTorch, Language = C++/Java, Compute Platform = CUDA 11.8 or 12.1
2. Extract to `C:\libtorch` (or your preferred location)
3. Set environment variable:
   ```cmd
   set LIBTORCH=C:\libtorch
   set LIBTORCH_USE_PYTORCH=1
   ```
   Or add permanently via System Properties → Environment Variables

#### Linux:
```bash
# Download LibTorch
wget https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.1.0%2Bcu118.zip
unzip libtorch-cxx11-abi-shared-with-deps-2.1.0+cu118.zip -d ~/

# Set environment variable
export LIBTORCH=~/libtorch
export LIBTORCH_USE_PYTORCH=1
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```

### Option 1: Run the Example (Recommended)

```bash
# Make sure you're in the project root
cd /path/to/Rust-AI-programmer

# Run the text generation example (uses CUDA GPU)
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
- **GPU (LibTorch/CUDA)**: ~20-80+ tokens/second (depends on your GPU)
- This implementation uses CUDA by default for maximum performance

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
1. Make sure you used `--release` flag
2. Verify CUDA is working: `nvidia-smi` should show your GPU
3. Check LibTorch environment variable is set correctly
4. Wait for first token (prompt processing takes longer)

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
