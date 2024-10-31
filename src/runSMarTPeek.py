import subprocess
import tempfile
import re
import sys
import os
from pathlib import Path

# Specify the path to the Verus executable here
VERUS_PATH = os.getenv("VERUS_PATH")

# Visitor list verusFmt
VISITORS = "CoreVerusVisitor,SimpleVisitor,QuantifierVisitor".split(',')

def run_verus(file_path):
    script_dir = os.path.dirname(os.path.abspath(__file__))
    with tempfile.NamedTemporaryFile(dir=script_dir, delete=False, suffix=".log") as temp_file:
        temp_file_path = temp_file.name
    try:
        result = subprocess.run(
            [VERUS_PATH, "--log-all", file_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )

        if result.stdout is None and result.stderr is None:
            print("Error: No output captured from Verus. Please check the command or executable path.")
            return None, None

        output = result.stdout + result.stderr
        with open(temp_file_path, 'w') as temp_file:
            temp_file.write(output)

        return output, result.returncode
    except FileNotFoundError:
        print(f"Error: Verus not found at {VERUS_PATH}. Please check the path.")
        return None, None
    except Exception as e:
        print(f"An error occurred: {e}")
        return None, None
        
def analyze_output(output):
    # Check if verification succeeded based on "0 errors" in the output
    if "0 errors" in output:
        return "Success", None

    # Try to identify the first failed verification case
    failure_pattern = re.compile(r"error\[(E\d+)\]: (.+)")
    for line in output.splitlines():
        match = failure_pattern.search(line)
        if match:
            error_code = match.group(1)
            error_message = match.group(2)
            return "Failure", f"{error_code}: {error_message}"

    return "Failure", "Unknown verification error."


def run_cargo(file_path):
    try:
        # Join VISITORS list into a comma-separated string
        visitors_string = ','.join(VISITORS)
        
        result = subprocess.run(
            ["cargo", "run", file_path, "--visitors", visitors_string],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        print("Cargo output:")
        print(result.stdout)
        # Uncomment the next line if you want to see error output as well
        # print(result.stderr)
        
        return result.stdout, result.returncode  # Return output and return code for further analysis
    except Exception as e:
        print(f"An error occurred while running Cargo: {e}")


def main(rust_file):
    if not Path(rust_file).is_file():
        print(f"Error: File '{rust_file}' not found.")
        return
    print("--------------------")
    print("Initial Verus Check")
    print("--------------------\n")

    output, returncode = run_verus(rust_file)
    if output is None:
        return

    with tempfile.NamedTemporaryFile(delete=False, mode="w", suffix=".log") as temp_file:
        temp_file.write(output)
        temp_filename = temp_file.name
        print(f"Output saved to temporary file: {temp_filename}")

    status, message = analyze_output(output)
    if status == "Success":
        print("Verification Result: Success")
    else:
        print("Verification Result: Failure")
        print("First Failed Case:", message)
        # Run Cargo with visitors if verification fails
        print("--------------------")
        print("Finitization Step")
        print("--------------------\n")
        
        run_cargo(rust_file)
        
        print("--------------------")
        print("Running Verus On Finitized System")
        print("--------------------\n")
        # Get the last visitor and its ID for the new Verus run
        if VISITORS:
            last_visitor = VISITORS[-1].strip()  # Get the last visitor
            visitor_count = VISITORS.count(last_visitor) - 1  # Count occurrences, adjust to start at 0
            # Create new filename for Verus
            input_file_stem = Path(rust_file).stem  # Get the stem of the input file
            new_file_name = f"{input_file_stem}_formatted_{last_visitor}_{visitor_count}.rs"
            new_file_path = Path(rust_file).parent / new_file_name  # Ensure the new file path is correct
            # Check if the new file exists
            verus_output, verus_returncode = run_verus(new_file_path)
            if not new_file_path.is_file():
                print(f"Error: The file '{new_file_path}' does not exist. Please check if it was created successfully.")
            else:
                # Run Verus again with the new visitor file
                print(f"Running Verus on: {new_file_path}")
                verus_output, verus_returncode = run_verus(new_file_path)

                # Analyze the output of the new Verus run
                if verus_output is not None:
                    verus_status, verus_message = analyze_output(verus_output)
                    if verus_status == "Success":
                        print("Verification Result (after re-check): Success")
                    else:
                        print("Verification Result (after re-check): Failure")
                        print("First Failed Case (after re-check):", verus_message)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python verify_rust.py <rust_file>")
    else:
        main(sys.argv[1])
