
import os
import sys
from PIL import Image

def generate_icons(input_path, output_dir):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    try:
        # Open and ensure RGBA
        img = Image.open(input_path).convert("RGBA")
        
        # Dimensions for Tauri
        sizes = {
            "32x32.png": (32, 32),
            "128x128.png": (128, 128),
            "128x128@2x.png": (256, 256),
            "icon.png": (512, 512),
            "icon.icns": (512, 512), # Simplified, usually requires checks
            "icon.ico": (256, 256)   # Simplified
        }

        for filename, size in sizes.items():
            out_path = os.path.join(output_dir, filename)
            resized = img.resize(size, Image.Resampling.LANCZOS)
            
            # Save based on extension
            if filename.endswith(".icns"):
                # ICNS isn't directly supported by PIL save usually without formatting, 
                # but Tauri often just needs the file to exist or use .png. 
                # For safety, we save as PNG for the others.
                # Actually, for standard Tauri dev 2.0, the pngs are most important.
                # Let's save as PNG content even if named .icns for now if PIL supports, 
                # or just skip complex ICNS generation and rely on PNGs which Tauri 2 accepts.
                # However, to avoid "invalid PNG signature" if it expects PNG content:
                resized.save(out_path, format="PNG")
            elif filename.endswith(".ico"):
                resized.save(out_path, format="ICO")
            else:
                resized.save(out_path, format="PNG")
            
            print(f"Generated {out_path}")

        print("Icon generation complete.")

    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python3 generate_icons.py <input_path> <output_dir>")
        sys.exit(1)
    
    generate_icons(sys.argv[1], sys.argv[2])
