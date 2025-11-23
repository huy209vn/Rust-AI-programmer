from safetensors.torch import load_file
import torch
import os

model_dir = r"C:\Users\PC\Documents\Rusta\rusta-base-model"

for shard_id in range(1, 7):
    in_file = os.path.join(model_dir, f"model-0000{shard_id}-of-00006.safetensors")
    out_file = in_file.replace(".safetensors", ".pt")

    print("Converting:", in_file)
    tensors = load_file(in_file)
    torch.save(tensors, out_file)

print("Done.")