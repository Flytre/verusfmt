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
    
    # Create the temp file for log output
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
            return None, temp_file_path

        # Combine stdout and stderr
        output = result.stdout + result.stderr
        
        # Write the output to the temp file
        with open(temp_file_path, 'w') as temp_file:
            temp_file.write(output)

        return output, temp_file_path  # Return output and temp file path for main
    except FileNotFoundError:
        print(f"Error: Verus not found at {VERUS_PATH}. Please check the path.")
        return None, temp_file_path
    except Exception as e:
        print(f"An error occurred: {e}")
        return None, temp_file_path


        
def analyze_output(output):
    # Check for any previous errors that lead to abortion
    abort_pattern = re.compile(r"error: aborting due to (\d+) previous errors")
    if abort_pattern.search(output):
        return "Failure", None, None, "Aborted due to previous errors."

    # Check for successful verification based on "0 errors" in the output
    if "0 errors" in output:
        return "Success", None, None, None

    # Match the failure pattern for assertion failures
    failure_pattern = re.compile(
        r"error: assertion failed\s+--> (.*?):(\d+):\d+\s+"
        r".*?\n\s*(\d+ \|.*?)\n\s*\|\s*(\S+.*?) assertion failed",
        re.DOTALL
    )

    # Search the output for the first match
    match = failure_pattern.search(output)
    if match:
        # Extract relevant parts from the regex match
        file_name = match.group(1)
        line_number = match.group(2)
        assertion_code = match.group(3).strip()  # Assertion line
        
        # Shorten assertion code to just the relevant part
        assertion_code = assertion_code.split('|')[-1].strip()  # Get the part after the pipe and trim whitespace
        
        # Return as separate variables
        return "Failure", file_name, line_number, assertion_code

    # Return "unknown verification error" if no matches were found
    return "Failure", None, None, "Unknown verification error."



def run_cargo(file_path, assertion_code=None):
    try:
        # Join VISITORS list into a comma-separated string
        visitors_string = ','.join(VISITORS)

        # Build the command arguments, including the optional assertion code
        cargo_command = ["cargo", "run", file_path, "--visitors", visitors_string]
        
        # If assertion_code is provided, add it to the command
        if assertion_code:
            cargo_command.extend(["--assertion-code", assertion_code])

        result = subprocess.run(
            cargo_command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        print("Cargo output:")
        print(result.stdout)
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

    # Run Verus on the initial file
    output, returncode = run_verus(rust_file)
    if output is None:
        return

    status, file_name, line_number, assertion_code = analyze_output(output)

    if status == "Failure" and assertion_code == "Aborted due to previous errors.":
        print("Verification aborted due to previous errors. Exiting script.")
        return  # Exit the script if aborted due to errors

    if status == "Success":
        print("Verification Result: Success")
    else:
        print("Verification Result: Failure")
        print("First Failed Case:")
        
        # Display the detailed failure information, if available
        if file_name and line_number and assertion_code:
            print(f"{file_name}:{line_number} - {assertion_code}")
            print(assertion_code)
        else:
            print("Unknown verification error.")
        
        # Run Cargo with visitors if verification fails
        print("--------------------")
        print("Finitization Step")
        print("--------------------\n")
        
        run_cargo(rust_file,assertion_code)
        
        print("--------------------")
        print("Running Verus On Finitized System")
        print("--------------------\n")
        
        # Get the last visitor and its ID for the new Verus run
        if VISITORS:
            last_visitor = VISITORS[-1].strip()  # Get the last visitor
            visitor_count = VISITORS.count(last_visitor) - 1  # Count occurrences, adjust to start at 0
            
            # Create a new filename for Verus based on visitor and count
            input_file_stem = Path(rust_file).stem  # Get the stem of the input file
            new_file_name = f"{input_file_stem}_formatted_{last_visitor}_{visitor_count}.rs"
            new_file_path = Path(rust_file).parent / new_file_name  # Ensure the new file path is correct
            
            # Check if the new file exists before running Verus on it
            if not new_file_path.is_file():
                print(f"Error: The file '{new_file_path}' does not exist. Please check if it was created successfully.")
                return
            
            # Run Verus again with the new visitor file
            print(f"Running Verus on: {new_file_path}")
            verus_output, verus_returncode = run_verus(new_file_path)
            
            # Analyze the output of the new Verus run
            if verus_output is not None:
                verus_status, verus_file_name, verus_line_number, verus_assertion_code = analyze_output(verus_output)
                if verus_status == "Success":
                    print("Verification Result (after re-check): Success")
                else:
                    print("Verification Result (after re-check): Failure")
                    print("First Failed Case (after re-check):")
                    if verus_file_name and verus_line_number and verus_assertion_code:
                        print(f"{verus_file_name}:{verus_line_number} - {verus_assertion_code}")
                    else:
                        print("Unknown verification error (after re-check).")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python verify_rust.py <rust_file>")
    else:
        main(sys.argv[1])
