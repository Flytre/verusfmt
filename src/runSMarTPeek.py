import subprocess
import tempfile
import re
import sys
import os
from pathlib import Path
import argparse

# Specify the path to the Verus executable here
VERUS_PATH = os.getenv("VERUS_PATH")

# Visitor list verusFmt
VISITORS = "QuantifierVisitor,LoopVisitor,RangeBoundsVisitor".split(',')


def run_verus(file_path):
    # Determine the directory and filename of the input file
    file_dir = Path(file_path).parent
    file_stem = Path(file_path).stem
    
    # Create the 'logs' directory within the script's directory if it doesn't exist
    script_dir = Path(__file__).parent  # Directory of the current script
    logs_dir = script_dir / "logs"
    logs_dir.mkdir(exist_ok=True)
    
    # Set the temp file path in the 'logs' directory, with a name based on the input file
    temp_file_path = logs_dir / f"{file_stem}_verus.log"
    print(f"verus log created at: {temp_file_path}")
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

def run_verus_on_finitized_system(rust_file, type, bound=None):
    if bound is None:
        print("--------------------")
        print(f"Running Verus - {type} - On Finitized System")
        print("--------------------\n")
    else:
        print("--------------------")
        print(f"Running Verus - {type} - On Finitized System -- BOUND = {bound}")
        print("--------------------\n")

    # Get the last visitor and its ID for the new Verus run
    if VISITORS:
        last_visitor = VISITORS[-1].strip()  # Get the last visitor
        visitor_count = VISITORS.count(last_visitor) - 1  # Count occurrences, adjust to start at 0
        
        # Create a new filename based on visitor, count, and optionally bound
        input_file_stem = Path(rust_file).stem  # Get the stem of the input file
        if bound is not None:
            new_file_name = f"{input_file_stem}_formatted_{last_visitor}_{visitor_count}_bound_{bound}.rs"
        else:
            new_file_name = f"{input_file_stem}_formatted_{last_visitor}_{visitor_count}.rs"
        
        # Construct the new file path
        new_file_path = Path(rust_file).parent / new_file_name
        if not str(new_file_path).startswith("tempFiles"):
            new_file_path = Path(rust_file).parent / "tempFiles" / new_file_name
        
        # Check if the new file exists before running Verus on it
        if not new_file_path.is_file():
            print(f"Error: The file '{new_file_path}' does not exist. Please check if it was created successfully.")
            return None, None, None  # Return None if the file doesn't exist
            
        # Run Verus with the new visitor file
        print(f"Running Verus on: {new_file_path}")
        output, returncode = run_verus(new_file_path)
        
        # Analyze the output of the Verus run
        status, assertion_code, failure_type = handle_verus_output(output) 
        if assertion_code:
            print(f"Failed with assertion code = {assertion_code}")
    
        print("--------------------\n")
        return status, assertion_code, failure_type  # Return the results for further processing



def handle_verus_output(output):
    if output is None:
        return None, None, None  # Return None for all if output is None

    status, file_name, line_number, assertion_code, failure_type = analyze_output(output)

    if status == "Failure" and (assertion_code == "Aborted due to previous errors with no verified results." or assertion_code == "Type Mismatch"):
        print("Verification aborted due to previous errors. Exiting script.")
        return status, None, None  # Return status and None for others to indicate failure

    if status == "Success":
        print("Verification Result: Success\n")
    else:
        print("Verification Result: Failure")
        print("First Failed Case:")
        
        if file_name and line_number and assertion_code and failure_type:
            print(f"{file_name}:{line_number} - {assertion_code} :: {failure_type}")
            print(assertion_code)
        else:
            print("Unknown verification error.")
    
    return status, assertion_code, failure_type  # Return status, assertion_code, and failure_type



        
def analyze_output(output):
    # Check for aborting due to previous errors with no verification results
    abort_pattern = re.compile(r"error: aborting due to \d+ previous error[s]*;")
    verification_results_pattern = re.compile(r"verification results:: 0 verified, 0 errors")

    if abort_pattern.search(output) and verification_results_pattern.search(output):
        return "Failure", None, None, "Aborted due to previous errors with no verified results.", None

    # Check for mismatched types error
    mismatched_types_pattern = re.compile(
        r"error\[E0308\]: mismatched types\s*"
        r"--> (.*?):(\d+):\d+\s*"
        r"\|\s*(.*?)\n", re.DOTALL
    )
    match = mismatched_types_pattern.search(output)
    if match:
        file_name = match.group(1).strip()
        line_number = match.group(2).strip()
        error_detail = match.group(3).strip()  # Extracts detail of the mismatched type message
        return "Failure", file_name, line_number, error_detail, "Type Mismatch"

    # Check for successful verification based on "0 errors" in the output
    if "0 errors" in output:
        return "Success", None, None, None, None

    # Define patterns for assertion, postcondition, and loop invariant errors
    error_patterns = {
        "Assertion": re.compile(
            r"error: assertion failed\s+--> (.*?):(\d+):\d+\s+"
            r".*?\n\s*(\d+ \|.*?)\n\s*\|\s*(.*?)(?=\s+assertion failed)", re.DOTALL
        ),
        "Postcondition": re.compile(
            r"error: postcondition not satisfied\s+--> (.*?):(\d+):\d+\s+"
            r".*?\n\s*(\d+ \|.*?)\n\s*\|\s*(.*?) failed this postcondition", re.DOTALL
        ),
        "LoopInvariant": re.compile(
            r"error: invariant not satisfied (?:at end of loop body|before loop)\s+--> (.*?):(\d+):\d+\s+"
            r".*?\n\s*(\d+ \|.*?)\n\s*\|\s*(.*?)", re.DOTALL
        )
    }

    # Initialize to find the first error occurrence
    first_error_type, first_error_match, first_error_position = None, None, float('inf')

    # Search for each error type and determine the earliest one
    for error_type, pattern in error_patterns.items():
        match = pattern.search(output)
        if match and match.start() < first_error_position:
            first_error_type = error_type
            first_error_match = match
            first_error_position = match.start()

    # Return the earliest matched error type
    if first_error_match:
        file_name = first_error_match.group(1).strip()
        line_number = first_error_match.group(2).strip()
        error_code = first_error_match.group(3).strip().split('|')[-1].strip()
        return "Failure", file_name, line_number, error_code, first_error_type

    # Return "unknown verification error" if no matches were found
    return "Failure", None, None, "Unknown verification error.", None




def run_cargo(file_path, assertion_code=None, visitors=None, bound=None):
    file_path_str = str(file_path) 

    # Join VISITORS list into a comma-separated string if visitors are provided
    if visitors is not None:
        visitors_string = ','.join(visitors)
    else:
        visitors_string = ','.join(VISITORS)

    # Build the command arguments, including the optional assertion code
    cargo_command = ["cargo", "run", file_path_str, "--visitors", visitors_string]
    # Add bound to the command if it's provided
    if bound is not None:
        cargo_command.extend(["--bound", str(bound)])

    # If assertion_code is provided, add it to the command
    if assertion_code:
        if assertion_code.startswith("invariant"):
            assertion_code = assertion_code[len("invariant"):].strip()  # Remove "invariant" prefix if it exists
        cargo_command.extend(["--assertion-code", assertion_code])

    # Execute the command and capture the output
    result = subprocess.run(
        cargo_command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    print("Cargo output:")
    print(result.stdout)
    return result.stdout, result.returncode  # Return output and return code for further analysis




def singleFullPass(rust_file, mode='Full', bound=None, iterative=False):
    if mode in ["Full", "Impl"]:
        print("--------------------")
        print("Running Cargo with StripProofVisitor")
        print("--------------------\n")
        
        run_cargo(rust_file, visitors=["StripProofVisitor"])
        
        print("--------------------")
        print("Verus Check -- Stripped Impl")
        print("--------------------\n")

        input_file_stem = Path(rust_file).stem
        new_file_name = f"{input_file_stem}_formatted_StripProofVisitor_0.rs"
        new_file_path = Path(rust_file).parent / "./tempFiles" / new_file_name

        if not new_file_path.is_file():
            print(f"Error: The file '{new_file_path}' does not exist.")
            return
        
        # Run Verus on the formatted file
        print(f"Running Verus on: {new_file_path}")
        verus_output, verus_returncode = run_verus(new_file_path)
        status, assertion_code, failure_type = handle_verus_output(verus_output)

        if not (status == "Success" and assertion_code is None):
            print("Finitizing Impl..")
            print("\n--------------------")
            print("Finitization (Impl) Step")
            print("--------------------\n")
            run_cargo(new_file_path, assertion_code)
            status, assertion_code, failure_type = run_verus_on_finitized_system(new_file_path, "Impl Only")

            if status == "Failure":
                print("IMPL IS INCORRECT")
                return

    print("----------------------------------------\n")

    if mode in ["Full", "Proof"]:
        print("--------------------")
        print("Verus Check - Original File")
        print("--------------------\n")

        verus_output, verus_returncode = run_verus(rust_file)
        status, assertion_code, failure_type = handle_verus_output(verus_output)
        
        if assertion_code:
            print("--------------------")
            print("Finitization (proof) Step")
            print("--------------------\n")
            run_cargo(rust_file, assertion_code)
            run_verus_on_finitized_system(rust_file, "Proof")



def iterativePass(rust_file, mode='Full', bound=None):
    print("Starting iterative verification...")
    if mode in ["Full", "Impl"]:
        print("--------------------")
        print("Running Cargo with StripProofVisitor")
        print("--------------------\n")
        
        run_cargo(rust_file, visitors=["StripProofVisitor"])
        
        print("--------------------")
        print("Verus Check -- Stripped Impl")
        print("--------------------\n")

        input_file_stem = Path(rust_file).stem
        new_file_name = f"{input_file_stem}_formatted_StripProofVisitor_0.rs"
        new_file_path = Path(rust_file).parent / "./tempFiles" / new_file_name

        if not new_file_path.is_file():
            print(f"Error: The file '{new_file_path}' does not exist.")
            return
        

        # Run Verus on the formatted file
        print(f"Running Verus on: {new_file_path}")
        verus_output, verus_returncode = run_verus(new_file_path)
        status, assertion_code, failure_type = handle_verus_output(verus_output)

        if not (status == "Success" and assertion_code is None):
            print("Finitizing Impl..")
            print("\n--------------------")
            print("Finitization (Impl) Step")
            print("--------------------\n")

            # Loop from 1 to bound, passing each loop value to run_cargo
        for i in range(1, bound + 1):
            print(f"Running Cargo with bound = {i}")
            run_cargo(new_file_path, assertion_code, bound=i)  # Pass the current bound iteration value
            
            status, assertion_code, failure_type = run_verus_on_finitized_system(new_file_path, "Impl Only", bound=i)
            
            # Check the status after each run to exit early if verification fails
            if status == "Failure":
                print("IMPL IS INCORRECT")
                break  # Exit the loop on failure
            elif status == "Success":
                print(f"Attempt {i} succeeded. Continuing to next iteration...")

        else:
            # Only reached if the loop completes without breaking (no failure in any iteration)
            print("Impl verification succeeded after all attempts.")

        print("----------------------------------------\n")

        if mode in ["Full", "Proof"]:
            print("--------------------")
            print("Verus Check - Original File")
            print("--------------------\n")
       
            verus_output, verus_returncode = run_verus(rust_file)
            status, assertion_code, failure_type = handle_verus_output(verus_output)
        
            if assertion_code:
                print("--------------------")
                print("Finitization (proof) Step")
                print("--------------------\n")
                for i in range(1, bound + 1):
                    print(f"Running Cargo with bound = {i}")
                    run_cargo(rust_file, assertion_code, bound=i)  # Pass the current bound iteration value
                    run_verus_on_finitized_system(rust_file, "Proof", bound=i)
                    # status, assertion_code, failure_type = run_verus_on_finitized_system(new_file_path, "Impl Only", bound=i)



    

def main(rust_file, mode='Full', bound=None, iterative=False):
    if not Path(rust_file).is_file():
        print(f"Error: File '{rust_file}' not found.")
        return
    
    if iterative:
        iterativePass(rust_file, mode=mode, bound=bound)
    else:
        singleFullPass(rust_file, mode=mode, bound=bound, iterative=iterative)



if __name__ == "__main__":
    # Set up command-line argument parsing
    parser = argparse.ArgumentParser(description='Verify Rust files using Verus.')
    parser.add_argument('--mode', type=str, choices=['Full', 'Proof', 'Impl'], default='Full', 
                        help='Mode of verification (default: Full).')
    parser.add_argument('--bound', type=int, default=None, 
                        help='Optional bound parameter for verification (default: None).')
    parser.add_argument('--iterative', action='store_true', 
                        help='Enable iterative mode for verification (requires --bound).')

    # Collect all arguments except the last one as input
    args, input_file = parser.parse_known_args()  # Parse the arguments

    # Check if --iterative is specified without --bound
    if args.iterative and args.bound is None:
        parser.error("--iterative requires --bound to be specified.")

    # Pass the arguments to main
    main(input_file[-1], mode=args.mode, bound=args.bound, iterative=args.iterative)
