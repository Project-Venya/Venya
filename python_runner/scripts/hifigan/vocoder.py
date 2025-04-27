import os
import json
import torch
import numpy as np
from scipy.io.wavfile import write
from env import AttrDict
from meldataset import MAX_WAV_VALUE
from models import Generator


def load_checkpoint(filepath, device):
    if not os.path.isfile(filepath):
        raise FileNotFoundError(f"Checkpoint not found at {filepath}")
    print(f"Loading checkpoint from: {filepath}")
    checkpoint_dict = torch.load(filepath, map_location=device)
    return checkpoint_dict


def inference_from_mel(checkpoint_path, mel_path, output_wav_path):
    # Load config
    config_file = os.path.join(os.path.dirname(checkpoint_path), 'config.json')
    if not os.path.isfile(config_file):
        raise FileNotFoundError(f"Config file not found at {config_file}")

    with open(config_file) as f:
        config_data = json.load(f)

    h = AttrDict(config_data)
    torch.manual_seed(h.seed)

    device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
    if torch.cuda.is_available():
        torch.cuda.manual_seed(h.seed)

    # Load generator model
    generator = Generator(h).to(device)
    checkpoint = load_checkpoint(checkpoint_path, device)
    generator.load_state_dict(checkpoint['generator'])

    generator.eval()
    generator.remove_weight_norm()

    # Load mel spectrogram
    mel = np.load(mel_path)
    mel_tensor = torch.FloatTensor(mel).to(device)

    with torch.no_grad():
        y_g_hat = generator(mel_tensor)
        audio = y_g_hat.squeeze().cpu().numpy()
        audio = (audio * MAX_WAV_VALUE).astype(np.int16)

    # Save audio
    os.makedirs(os.path.dirname(output_wav_path), exist_ok=True)
    write(output_wav_path, h.sampling_rate, audio)
    print(f"Generated audio saved to: {output_wav_path}")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--checkpoint", required=True, help="Path to the model checkpoint (.pt)")
    parser.add_argument("--mel", required=True, help="Path to the input mel .npy file")
    parser.add_argument("--output", required=True, help="Path to save the generated .wav file")
    args = parser.parse_args()

    inference_from_mel(args.checkpoint, args.mel, args.output)
