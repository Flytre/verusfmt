import subprocess

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
set datafile separator ','
set xtics auto
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


if __name__ == "__main__":
    # Path to the CSV file and output image
    csv_file = './experiment_logs/experiment_summary_n.csv'
    plot_image = './experiment_logs/experiment_plot.png'

    # Generate gnuplot script
    gnuplot_script = generate_gnuplot_script(csv_file, plot_image)

    # Run gnuplot with the generated script
    try:
        subprocess.run(['gnuplot', gnuplot_script], check=True)
        print(f"Plot saved to {plot_image}")
    except subprocess.CalledProcessError as e:
        print(f"Error running gnuplot: {e}")
