import subprocess
import os
import argparse
import csv
import json
from itertools import combinations
import time

def debug_print(debug_enabled, message):
    """Print a debug message if debugging is enabled."""
    if debug_enabled:
        print(message)

def run_cargo(file, visitors, experiment_name, working_dir, debug_enabled):
    """Run the cargo command."""
    cargo_command = [
        "cargo", "run", "--",
        f"--file={file}",
        f"--visitors={visitors}",
        f"--experiment-name={experiment_name}",
    ]
    debug_print(debug_enabled, f"Running command: {' '.join(cargo_command)}")
    try:
        result = subprocess.run(
            cargo_command,
            cwd=working_dir,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        debug_print(debug_enabled, result.stdout)
    except subprocess.CalledProcessError as e:
        print("Error while running cargo run:")
        print(e.stderr)

def generate_diff_csv(experiments_dir, output_csv, debug_enabled):
    """Run diff on all combinations of files and record results in a CSV file."""
    debug_print(debug_enabled, f"DEBUG: Experiments directory: {experiments_dir}")
    files = [f for f in os.listdir(experiments_dir) if f.endswith(".rs")]
    debug_print(debug_enabled, f"DEBUG: Found files: {files}")

    file_pairs = list(combinations(files, 2))
    debug_print(debug_enabled, f"DEBUG: File pairs to compare: {file_pairs}")

    diff_results = []

    for file1, file2 in file_pairs:
        file1_path = os.path.join(experiments_dir, file1)
        file2_path = os.path.join(experiments_dir, file2)

        debug_print(debug_enabled, f"DEBUG: Comparing {file1_path} and {file2_path}")
        try:
            diff_result = subprocess.run(
                ["diff", file1_path, file2_path],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )
            same = diff_result.returncode == 0
            debug_print(debug_enabled, f"DEBUG: Result for {file1} vs {file2}: {'Same' if same else 'Different'}")
            
            # Only record if the files are different
            if not same:
                diff_results.append([file1, file2, "No"])
        except Exception as e:
            print(f"Error running diff between {file1} and {file2}: {e}")

    # Return the diff results to be written later
    return diff_results

def execute_experiment(experiment, working_dir):
    """Run a single experiment."""
    program = experiment["program"]
    visitors = ",".join(experiment["visitors"])
    args = experiment.get("args", [])
    experiment_name = experiment["name"]
    debug_enabled = "--debug" in args

    # Define experiment directories
    experiments_dir = os.path.abspath(
        os.path.join(working_dir, "../permutation_experiments", experiment_name)
    )
    os.makedirs(experiments_dir, exist_ok=True)
    output_csv = os.path.join(experiments_dir, "diff_results.csv")

    # Run cargo and record time
    print(f"Starting experiment: {experiment['name']}")
    start_time = time.time()
    run_cargo(program, visitors, experiment_name, working_dir, debug_enabled)
    cargo_time = time.time() - start_time
    print("[DONE]\n")

    # Run diff generation and record time
    diff_time = None
    diff_results = []
    if os.path.exists(experiments_dir):
        print(f"Generating diff results in {output_csv}")
        start_time = time.time()
        diff_results = generate_diff_csv(os.path.join(experiments_dir, os.path.basename(program)), output_csv, debug_enabled)
        diff_time = time.time() - start_time
        print(f"[DONE]: Diff results saved to {output_csv}")
    else:
        print(f"Experiments directory {experiments_dir} does not exist. Skipping diff generation.")

    # Write results to the CSV file
    with open(output_csv, mode="w", newline="") as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(["Cargo Execution Time (seconds)", f"{cargo_time:.4f}"])
        writer.writerow(["Diff Generation Time (seconds)", f"{diff_time:.4f}" if diff_time is not None else "N/A"])
        writer.writerow([])  # Blank line for separation
        writer.writerow(["File 1", "File 2", "Same?"])  # Header row
        writer.writerows(diff_results)

def main():
    # Parse command-line arguments
    parser = argparse.ArgumentParser(description="Wrapper script to run vpermute experiments.")
    parser.add_argument("--config", required=True, help="Path to the JSON configuration file.")
    args = parser.parse_args()

    # Load configuration
    with open(args.config, "r") as config_file:
        config = json.load(config_file)

    # Working directory
    working_dir = os.getcwd()

    # Execute each experiment sequentially
    for experiment in config["experiments"]:
        execute_experiment(experiment, working_dir)

if __name__ == "__main__":
    main()
