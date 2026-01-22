import os
import sys
import glob

def read_leb128(data, pos):
    result = 0
    shift = 0
    while True:
        b = data[pos]
        pos += 1
        result |= (b & 0x7f) << shift
        if not (b & 0x80):
            break
        shift += 7
    return result, pos

def read_string(data, pos):
    length, pos = read_leb128(data, pos)
    s = data[pos:pos+length].decode('utf-8', errors='replace')
    return s, pos + length

def analyze_wasm(path):
    print(f"Analyzing {path}...")
    with open(path, 'rb') as f:
        data = f.read()

    if data[:4] != b'\x00asm':
        print("Not a WASM file")
        return

    pos = 8 # Skip magic and version
    
    leaks = []
    
    while pos < len(data):
        section_id = data[pos]
        pos += 1
        section_len, pos = read_leb128(data, pos)
        section_end = pos + section_len
        
        if section_id == 2: # Import section
            count, pos = read_leb128(data, pos)
            for _ in range(count):
                mod, pos = read_string(data, pos)
                field, pos = read_string(data, pos)
                kind = data[pos]
                pos += 1
                if kind == 0: # Function
                    _, pos = read_leb128(data, pos)
                elif kind == 1: # Table
                    pos += 1 # element type
                    _, pos = read_leb128(data, pos) # flags
                    _, pos = read_leb128(data, pos) # initial
                elif kind == 2: # Memory
                    _, pos = read_leb128(data, pos) # flags
                    _, pos = read_leb128(data, pos) # initial
                elif kind == 3: # Global
                    pos += 2 # type and mutability
                
                if mod == "env":
                    leaks.append(field)
        else:
            pos = section_end

    if leaks:
        print("\n[!] Found 'env' imports (Leaks):")
        for leak in sorted(set(leaks)):
            print(f"  - {leak}")
    else:
        print("\n[✓] No 'env' imports found.")

if __name__ == "__main__":
    dist_dir = "/Users/vegarberentsen/Documents/PeakOS/crates/modes/desktop/dist"
    wasm_files = glob.glob(os.path.join(dist_dir, "*.wasm"))
    if not wasm_files:
        print(f"No WASM files found in {dist_dir}")
        sys.exit(1)
    
    # Analyze the most recent one
    latest_wasm = max(wasm_files, key=os.path.getmtime)
    analyze_wasm(latest_wasm)
