import os
import sys
import json
import subprocess
import re
import argparse
from pathlib import Path

def parse_log(log_file_path):
    impl_result = "N/A"
    proof_result = "N/A"
    impl_proof_time = "N/A"
    proof_proof_time = "N/A"
    impl_finitize_time = "N/A"
    proof_finitize_time = "N/A"
    
    # Read the log file
    with open(log_file_path, 'r') as log_file:
        log_content = log_file.read()

    # Look for the "Verus Check -- Stripped Impl" block
    impl_block_start = "Finitization (Impl) Step"
    impl_result_start = "Verification Result: "

    # Find the Impl section
    if impl_block_start in log_content:
        impl_section = log_content.split(impl_block_start)[1]  # Get everything after the Impl block
        if impl_result_start in impl_section:
            # Extract result for the Impl block
            if "Verification Result: Success" in impl_section:
                impl_result = "Success"
            elif "Verification Result: Failure" in impl_section:
                impl_result = "Failure"
        
        # Extract the total time for the Impl block
        time_match = re.search(r"total time\s*=\s*(\d+)", impl_section)
        if time_match:
            impl_proof_time = time_match.group(1)

        # Extract the finitize time for Impl
        finitize_match = re.search(r"Time taken for `SMartPeek`:\s*([\d.]+)\s*ms", impl_section)
        if finitize_match:
            impl_finitize_time = finitize_match.group(1)

    # Look for the "Verus Check - Original File" block
    proof_block_start = "Finitization (proof) Step"

    # Find the Proof section
    if proof_block_start in log_content:
        proof_section = log_content.split(proof_block_start)[1]  # Get everything after the Proof block
        if impl_result_start in proof_section:
            # Extract result for the Proof block
            if "Verification Result: Success" in proof_section:
                proof_result = "Success"
            elif "Verification Result: Failure" in proof_section:
                proof_result = "Failure"
        
        # Extract the total time for the Proof block
        time_match = re.search(r"total time\s*=\s*(\d+)", proof_section)
        if time_match:
            proof_proof_time = time_match.group(1)

        # Extract the finitize time for Proof
        finitize_match = re.search(r"Time taken for `SMartPeek`:\s*([\d.]+)\s*ms", proof_section)
        if finitize_match:
            proof_finitize_time = finitize_match.group(1)

    return impl_result, proof_result, impl_proof_time, proof_proof_time, impl_finitize_time, proof_finitize_time


def generate_table(log_dir, output_csv, experiment_names):
    """Generate a table from log files in the specified directory."""
    # Prepare the rows for the table
    rows = [["Name", "Impl", "Proof", "Impl Proof Time (ms)", "Proof Proof Time (ms)", "Impl Finitize (ms)", "Proof Finitize (ms)"]]

    # Process only log files matching experiment names
    for name in experiment_names:
        log_file = f"{name}_program.txt"
        log_file_path = os.path.join(log_dir, log_file)

        # Ensure the log file exists
        if os.path.exists(log_file_path):
            impl_result, proof_result, impl_proof_time, proof_proof_time, impl_finitize_time, proof_finitize_time = parse_log(log_file_path)
            rows.append([name, impl_result, proof_result, impl_proof_time, proof_proof_time, impl_finitize_time, proof_finitize_time])
        else:
            print(f"Warning: Log file '{log_file}' not found for experiment '{name}'.")

    # Write the results to a CSV file
    with open(output_csv, 'w') as csv_file:
        for row in rows:
            csv_file.write(",".join(row) + "\n")

    print(f"Summary table written to {output_csv}")


def run_experiment(config_path, generate_table_only):
    # Ensure the config file exists
    if not os.path.exists(config_path):
        print(f"Error: Config file '{config_path}' not found.")
        sys.exit(1)

    # Load the configuration file
    with open(config_path, 'r') as config_file:
        config = json.load(config_file)

    # Ensure "experiments" key exists
    experiments = config.get("experiments", [])
    if not experiments:
        print("Error: No experiments found in the configuration file.")
        sys.exit(1)

    # Create the experiment_logs directory if it doesn't exist
    log_dir = "./experiment_logs"
    os.makedirs(log_dir, exist_ok=True)

    # Extract experiment names
    experiment_names = [experiment.get("name") for experiment in experiments if experiment.get("name")]

    # If not in generate-table-only mode, process each experiment
    if not generate_table_only:
        # Process each experiment
        for experiment in experiments:
            name = experiment.get("name")
            program = experiment.get("program")
            visitors = ",".join(experiment.get("visitors", []))
            args = " ".join(experiment.get("args", []))

            if not name or not program:
                print("Error: Each experiment must have a 'name' and 'program'. Skipping.")
                continue

            # Construct the command
            command = f"python3 ./src/runSMarTPeek.py {program} --visitors {visitors} {args}"
            print(f"Running command: {command}")

            # Run the command and capture the output
            try:
                result = subprocess.run(command, shell=True, capture_output=True, text=True)
                output = result.stdout
                error = result.stderr

                # Save the output to a log file
                log_file_path = os.path.join(log_dir, f"{os.path.splitext(name)[0]}_program.txt")
                with open(log_file_path, 'w') as log_file:
                    log_file.write(output)
                    if error:
                        log_file.write("\n--- Errors ---\n")
                        log_file.write(error)

                print(f"Output saved to {log_file_path}")
            except Exception as e:
                print(f"Error running command for experiment '{name}': {e}")

    # Generate the summary table
    output_csv = os.path.join(log_dir, "experiment_summary.csv")
    generate_table(log_dir, output_csv, experiment_names)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run experiments and generate log tables.")
    parser.add_argument("config_file", help="Path to the configuration file")
    parser.add_argument("--generate-table-only", action="store_true", help="Generate the table only, without running experiments")

    args = parser.parse_args()

    # Run the experiment with or without executing commands, based on the flag
    run_experiment(args.config_file, args.generate_table_only)
