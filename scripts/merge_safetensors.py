#!/usr/bin/env python3
"""
Merge sharded safetensors files into a single file for Burn to load.

Usage:
    python merge_safetensors.py MODEL_DIR

Example:
    python merge_safetensors.py "C:/Users/PC/.cache/huggingface/hub/models--Fortytwo-Network--Strand-Rust-Coder-14B-v1/snapshots/0b9a97c5ab89f9780c95356cc2ea121eb434372e"
"""

import json
import sys
from pathlib import Path
from safetensors import safe_open
from safetensors.torch import save_file

def merge_sharded_safetensors(model_dir: str):
    model_path = Path(model_dir)
    index_file = model_path / "model.safetensors.index.json"

    if not index_file.exists():
        print(f"Error: {index_file} not found!")
        sys.exit(1)

    print(f"Loading index from: {index_file}")
    with open(index_file) as f:
        index = json.load(f)

    # Get all shard files
    weight_map = index["weight_map"]
    shard_files = set(weight_map.values())

    print(f"Found {len(shard_files)} shard files:")
    for shard in sorted(shard_files):
        print(f"  - {shard}")

    # Load all tensors from all shards
    all_tensors = {}

    for shard_file in sorted(shard_files):
        shard_path = model_path / shard_file
        print(f"\nLoading {shard_file}...")

        with safe_open(shard_path, framework="pt") as f:
            for key in f.keys():
                tensor = f.get_tensor(key)
                all_tensors[key] = tensor
                print(f"  {key}: {tensor.shape}")

    # Save merged file
    output_file = model_path / "model.safetensors"
    print(f"\nSaving merged file to: {output_file}")
    print(f"Total tensors: {len(all_tensors)}")

    save_file(all_tensors, output_file)

    print(f"\n✓ Successfully merged {len(shard_files)} shards into {output_file}")
    print(f"✓ Total parameters: {len(all_tensors)} tensors")
    print(f"\nYou can now use: --model-path \"{model_dir}\"")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(1)

    model_dir = sys.argv[1]
    merge_sharded_safetensors(model_dir)
