import wave
import math
import struct
import random

def generate_click(filename):
    with wave.open(filename, 'w') as wav_file:
        # Set parameters: 1 channel, 2 bytes per sample, 44100 Hz
        wav_file.setnchannels(1)
        wav_file.setsampwidth(2)
        wav_file.setframerate(44100)
        
        # Generate a short click/blip
        # A quick envelope with some noise + sine wave
        duration = 0.05  # seconds
        num_samples = int(duration * 44100)
        
        for i in range(num_samples):
            t = i / 44100.0
            # Decaying volume envelope
            volume = 32000.0 * math.exp(-t * 50)
            
            # Mix of sine (tone) and random (noise) for a "mechanical" click
            # Base tone around 1500Hz dropping to 500Hz
            freq = 1500 - (i / num_samples) * 1000
            
            sample_val = math.sin(2 * math.pi * freq * t) * 0.7
            noise_val = (random.random() * 2 - 1) * 0.3
            
            value = int((sample_val + noise_val) * volume)
            value = max(-32768, min(32767, value))
            
            data = struct.pack('<h', value)
            wav_file.writeframes(data)

if __name__ == "__main__":
    generate_click('peak-native/assets/sounds/click.wav')
