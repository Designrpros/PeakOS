
from PIL import Image
import sys

def convert_to_rgba(path):
    try:
        img = Image.open(path)
        img = img.convert("RGBA")
        img.save(path)
        print(f"Converted {path} to RGBA")
    except Exception as e:
        print(f"Error converting {path}: {e}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        convert_to_rgba(sys.argv[1])
