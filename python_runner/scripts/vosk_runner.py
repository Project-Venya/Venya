# vosk_runner.py
import sys
import json
from vosk import Model, KaldiRecognizer  # type: ignore
import wave

def transcribe(wav_path):
    wf = wave.open(wav_path, "rb")

    if wf.getnchannels() != 1 or wf.getsampwidth() != 2 or wf.getframerate() != 16000:
        print(json.dumps({"error": "WAV must be mono, 16-bit, 16000 Hz"}))
        return

    model_path = os.path.abspath("../python_runner/resources/models/vosk-model-en-us-0.22-lgraph")
    model = Model(model_path)
    rec = KaldiRecognizer(model, wf.getframerate())

    result = []
    while True:
        data = wf.readframes(4000)
        if len(data) == 0:
            break
        if rec.AcceptWaveform(data):
            result.append(json.loads(rec.Result()))

    result.append(json.loads(rec.FinalResult()))

    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    import os
    wav_path = sys.argv[1]
    transcribe(wav_path)
