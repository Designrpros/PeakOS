import os

base_dir = "/Users/vegarberentsen/Documents/PeakOS/peak-native/assets/icons/menubar"
white_dir = os.path.join(base_dir, "white")
black_dir = os.path.join(base_dir, "black")

files = [f for f in os.listdir(base_dir) if f.endswith(".svg")]

for f in files:
    path = os.path.join(base_dir, f)
    with open(path, "r") as r:
        content = r.read()
    
    # Generate White
    white_content = content.replace("currentColor", "white")
    with open(os.path.join(white_dir, f), "w") as w:
        w.write(white_content)
        
    # Generate Black (Dark Grey for softness)
    black_content = content.replace("currentColor", "#323232")
    with open(os.path.join(black_dir, f), "w") as b:
        b.write(black_content)

print("Icons colorized.")
