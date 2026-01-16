from PIL import Image, ImageOps
import sys

def invert_image(input_path, output_path):
    try:
        # Open the image (ensure RGBA)
        img = Image.open(input_path).convert("RGBA")
        
        # Split channels
        r, g, b, a = img.split()
        
        # Invert RGB channels
        rgb_image = Image.merge("RGB", (r, g, b))
        inverted_rgb = ImageOps.invert(rgb_image)
        
        # Merge back with original Alpha
        r2, g2, b2 = inverted_rgb.split()
        final_img = Image.merge("RGBA", (r2, g2, b2, a))
        
        final_img.save(output_path, "PNG")
        print(f"Successfully inverted {input_path} to {output_path}")
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    invert_image(
        "/Users/vegarberentsen/Documents/PeakOS/peak-native/assets/icons/menubar/peak_logo.png",
        "/Users/vegarberentsen/Documents/PeakOS/peak-native/assets/icons/menubar/peak_logo_dark.png"
    )
