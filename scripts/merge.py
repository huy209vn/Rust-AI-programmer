from safetensors import safe_open
from safetensors.torch import save_file
import torch
import os
import sys

# Usage:
#   python merge_safetensors.py <model_dir> <output_file>

model_dir = sys.argv[1]
output_file = sys.argv[2]

# collect shard files
shards = sorted(
    [os.path.join(model_dir, f) for f in os.listdir(model_dir) if f.endswith(".safetensors")]
)

if not shards:
    print("No safetensor shards found.")
    exit(1)

print(f"Merging {len(shards)} shards...")
merged = {}

for shard in shards:
    print(f"Loading {shard}")
    with safe_open(shard, framework="pt") as f:
        for key in f.keys():
            if key in merged:
                raise ValueError(f"Duplicate tensor: {key}")
            merged[key] = f.get_tensor(key)

print(f"Saving merged file as {output_file}")
save_file(merged, output_file)
print("Done! ✔")