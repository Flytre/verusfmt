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

    # Combine all trial results into a single DataFrame and calculate the average
    if results:
        combined_df = pd.concat(results)
        averaged_df = combined_df.groupby(['Name', 'Impl', 'Proof'], as_index=False).mean()
        
        # Save the averaged result to a final CSV
        averaged_file = f'./experiment_logs/experiment_summary_{bound_value}_avg.csv'
        averaged_df.to_csv(averaged_file, index=False)

        return averaged_file
    return None

def generate_gnuplot_script(data_file, output_image):
    gnuplot_file = './experiment_logs/plot.gnuplot'
    script_content = f"""
set terminal pngcairo size 800,600 enhanced font 'Verdana,10'
set output '{output_image}'
set title 'Experiment Results'
set xlabel 'Bound'
set ylabel 'Time (ms)'
set grid
set key outside
plot \\
    '{data_file}' using 8:4 with linespoints title 'Impl Proof Time (ms)', \\
    '{data_file}' using 8:5 with linespoints title 'Proof Proof Time (ms)', \\
    '{data_file}' using 8:6 with linespoints title 'Impl Finitize (ms)', \\
    '{data_file}' using 8:7 with linespoints title 'Proof Finitize (ms)'
    """

    # Write the gnuplot script to a file
    with open(gnuplot_file, 'w') as f:
        f.write(script_content)

    return gnuplot_file



def main():
    if len(sys.argv) != 4:
        print("Usage: python run_experiments.py <config_file> <bound> <n_trials>")
        sys.exit(1)
    
    config_file = sys.argv[1]
    bound = int(sys.argv[2])
    n_trials = int(sys.argv[3])

    # Run experiments for bound values from 0 to the specified bound
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
        final_csv = './experiment_logs/experiment_summary_n.csv'
        final_df.to_csv(final_csv, index=False)

        # Generate the gnuplot script and run it
        # output_image = './experiment_logs/experiment_plot.png'
        # gnuplot_file = generate_gnuplot_script(final_csv, output_image)
        # subprocess.run(['gnuplot', gnuplot_file])
        # print(f"Plot generated: {output_image}")

if __name__ == "__main__":
    main()