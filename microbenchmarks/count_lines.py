import sys

def count_non_empty_lines(file_path):
    try:
        with open(file_path, 'r') as file:
            lines = file.readlines()
        
        # Flag to indicate whether we've encountered "verus! {"
        in_block = False
        count = 0

        for line in lines:
            line = line.strip()
            
            if not in_block:
                # Look for the "verus! {" line
                if line == "verus! {":
                    in_block = True
                continue
            
            # Count non-empty lines that don't start with "//"
            if line and not line.startswith("//"):
                count += 1
        
        return count
    
    except FileNotFoundError:
        print(f"File not found: {file_path}")
        return None
    except Exception as e:
        print(f"An error occurred: {e}")
        return None

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python count_verus_lines.py <file>")
        sys.exit(1)
    
    file_path = sys.argv[1]
    result = count_non_empty_lines(file_path)
    
    if result is not None:
        print(result)
