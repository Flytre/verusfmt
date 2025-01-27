import subprocess
import sys
import os

def generate_gnuplot_script(data_file, output_image, chart_title):
    gnuplot_file = './experiment_logs/plot.gnuplot'
    script_content = f"""
set terminal pngcairo size 3200,1600 enhanced font 'Verdana,16'
set output '{output_image}'
set title "{chart_title}" font 'Verdana,20'
set xlabel "Finitized Size" font 'Verdana,18'
set ylabel "Average Total Time (ms)" font 'Verdana,18'
set grid

# Set the key (legend) position to top left and flip it so colors are on the left
set key left top font 'Verdana,14'

# Set datafile separator
set datafile separator ","

# Plot the data with error bars and connect the points
plot "{data_file}" using 12:4:5 with errorbars title 'Impl Proof Time (ms)' lc rgb "blue" pointtype 5, \
     "{data_file}" using 12:6:7 with errorbars title 'Proof Proof Time (ms)' lc rgb "green" pointtype 6, \
     "{data_file}" using 12:8:9 with errorbars title 'Impl Finitize (ms)' lc rgb "orange" pointtype 7, \
     "{data_file}" using 12:10:11 with errorbars title 'Proof Finitize (ms)' lc rgb "red" pointtype 8, \
     "{data_file}" using 12:4:5 with linespoints lc rgb "blue" notitle, \
     "{data_file}" using 12:6:7 with linespoints lc rgb "green" notitle, \
     "{data_file}" using 12:8:9 with linespoints lc rgb "orange" notitle, \
     "{data_file}" using 12:10:11 with linespoints lc rgb "red" notitle
    """

    # Write the gnuplot script to a file
    with open(gnuplot_file, 'w') as f:
        f.write(script_content)

    return gnuplot_file


def main():
    if len(sys.argv) != 3:
        print("Usage: python generate_plot.py <csv_file> <chart_title>")
        sys.exit(1)

    csv_file = sys.argv[1]
    chart_title = sys.argv[2]

    # Derive plot image file name from CSV file
    plot_image = os.path.splitext(csv_file)[0] + '.png'

    # Generate gnuplot script
    gnuplot_script = generate_gnuplot_script(csv_file, plot_image, chart_title)

    # Run gnuplot with the generated script
    try:
        subprocess.run(['gnuplot', gnuplot_script], check=True)
        print(f"Plot saved to {plot_image}")
    except subprocess.CalledProcessError as e:
        print(f"Error running gnuplot: {e}")


if __name__ == "__main__":
    main()
