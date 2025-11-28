import torch
from transformers import AutoModelForCausalLM, AutoConfig
import os
import sys

# Usage:
#   python merge_qwen2_5_pt.py "C:\Users\PC\Documents\Rusta\rusta-base-model"

if len(sys.argv) < 2:
    print("Usage: python merge_qwen2_5_pt.py <model_dir>")
    sys.exit(1)

model_dir = sys.argv[1]
output_file = os.path.join(model_dir, "model.pt")

print("📁 Loading model from:", model_dir)

# 1. Load config (your uploaded qwen2_only_config.py matches this)
config = AutoConfig.from_pretrained(model_dir, trust_remote_code=True)

# 2. Load complete model + all shards
print("🧠 Loading all safetensor shards via HF (safe)...")
model = AutoModelForCausalLM.from_pretrained(
    model_dir,
    config=config,
    trust_remote_code=True,
    torch_dtype="auto",
    device_map=None
).cpu()   # Important: put model on CPU first

print("✓ Loaded all shards.")

# 3. Extract weights
state = model.state_dict()

print("💾 Saving merged PyTorch weights:", output_file)
torch.save(state, output_file)

print("\n🎉 Done! You now have a single merged file:")
print("    ", output_file)
print("Use this file in Burn's PyTorchFileRecorder.")