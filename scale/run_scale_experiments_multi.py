import os
import sys
import json
import subprocess
import shutil
import pandas as pd

def run_experiment(config_file, bound_value, n_trials):
    results = []

    for _ in range(n_trials):
        # Read and modify the config for the current bound value
        with open(config_file, 'r') as f:
            config = json.load(f)
        
        for experiment in config['experiments']:
            for i, arg in enumerate(experiment['args']):
                if arg.startswith('--bound'):
                    experiment['args'][i] = f'--bound {bound_value}'
        
        # Save the modified config to a temporary file
        temp_config = f'./temp_config_{bound_value}.json'
        with open(temp_config, 'w') as f:
            json.dump(config, f, indent=4)
        
        # Run the experiment using the modified config
        result = subprocess.run(['python', 'runExperiment.py', temp_config], capture_output=True, text=True)
        
        if result.returncode == 0:
            # Load the CSV result into a DataFrame
            df = pd.read_csv('./experiment_logs/experiment_summary.csv')
            results.append(df)
        else:
            print(f"Experiment failed for bound {bound_value}. Error: {result.stderr}")
        
        # Clean up the temporary config file
        os.remove(temp_config)

    # Combine all trial results into a single DataFrame and calculate the average and stdev
    if results:
        combined_df = pd.concat(results)
        # Group by the necessary columns and calculate both mean and stdev
        averaged_df = combined_df.groupby(['Name', 'Impl', 'Proof'], as_index=False).agg(
            {'Impl Proof Time (ms)': ['mean', 'std'],
             'Proof Proof Time (ms)': ['mean', 'std'],
             'Impl Finitize (ms)': ['mean', 'std'],
             'Proof Finitize (ms)': ['mean', 'std']}
        )
        
        # Flatten multi-level column headers
        averaged_df.columns = ['_'.join(col).strip() for col in averaged_df.columns.values]
        
        # Save the averaged result to a final CSV
        averaged_file = f'./experiment_logs/{os.path.splitext(os.path.basename(config_file))[0]}_{bound_value}_avg.csv'
        averaged_df.to_csv(averaged_file, index=False)

        return averaged_file
    return None


def main():
    if len(sys.argv) < 4:
        print("Usage: python run_experiments.py <config_file1> <config_file2> ... <bound> <n_trials>")
        sys.exit(1)
    
    config_files = sys.argv[1:-2]  # All arguments except the last two
    bound = int(sys.argv[-2])
    n_trials = int(sys.argv[-1])

    # Run experiments for each configuration file
    for config_file in config_files:
        summary_files = []
        for i in range(bound + 1):
            summary_file = run_experiment(config_file, i, n_trials)
            if summary_file:
                summary_files.append((i, summary_file))

        # Combine all averaged results into a single DataFrame for plotting
        all_results = []
        for bound_value, file in summary_files:
            df = pd.read_csv(file)
            df['Bound'] = bound_value
            all_results.append(df)

        if all_results:
            final_df = pd.concat(all_results)
            final_csv = f'./experiment_logs/{os.path.splitext(os.path.basename(config_file))[0]}_experiment_summary_n.csv'
            final_df.to_csv(final_csv, index=False)

if __name__ == "__main__":
    main()
