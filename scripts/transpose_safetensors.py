#!/usr/bin/env python3
"""
Transpose linear layer weights in safetensors files to match Burn's expectations.

PyTorch/HuggingFace stores Linear weights as [in_features, out_features]
Burn expects Linear weights as [out_features, in_features]

This script transposes the weights in-place.
"""

import argparse
from pathlib import Path
from safetensors import safe_open
from safetensors.torch import save_file
import torch


def transpose_weights(input_path: Path, output_path: Path):
    """Transpose linear layer weights in a safetensors file."""

    print(f"Loading: {input_path}")

    # Load tensors
    tensors = {}
    with safe_open(input_path, framework="pt") as f:
        for key in f.keys():
            tensors[key] = f.get_tensor(key)

    # Transpose linear layer weights (not embeddings or norms)
    transposed_count = 0
    for key, tensor in tensors.items():
        # Linear weights end with .weight and are 2D
        if key.endswith(".weight") and tensor.ndim == 2:
            # Skip embeddings and layer norms
            if "embed" not in key and "norm" not in key:
                print(f"  Transposing {key}: {list(tensor.shape)} -> {list(tensor.T.shape)}")
                tensors[key] = tensor.T.contiguous()
                transposed_count += 1

    print(f"Transposed {transposed_count} tensors")

    # Save transposed tensors
    print(f"Saving to: {output_path}")
    save_file(tensors, output_path)
    print("Done")


def main():
    parser = argparse.ArgumentParser(description="Transpose safetensors weights for Burn")
    parser.add_argument("input_dir", type=Path, help="Directory containing safetensors files")
    parser.add_argument("output_dir", type=Path, help="Directory to save transposed files")

    args = parser.parse_args()

    # Create output directory
    args.output_dir.mkdir(parents=True, exist_ok=True)

    # Find all safetensors files
    shard_files = sorted(args.input_dir.glob("*.safetensors"))

    if not shard_files:
        print(f"No safetensors files found in {args.input_dir}")
        return

    print(f"Found {len(shard_files)} shard(s)")
    print()

    # Process each shard
    for shard_path in shard_files:
        output_path = args.output_dir / shard_path.name
        transpose_weights(shard_path, output_path)
        print()

    print(f"All shards transposed and saved to {args.output_dir}")


if __name__ == "__main__":
    main()
