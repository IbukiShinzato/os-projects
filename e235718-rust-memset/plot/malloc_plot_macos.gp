set datafile separator comma
set terminal svg size 900,600
set output '../img/malloc_plot_macos.svg'
set xlabel "Memory Size (bytes)"
set ylabel "Time (ns)"
set logscale x 10
set style data linespoints
set grid
set key outside

plot "../data/malloc_data_macos.csv" skip 1 using 1:2 with linespoints title "Allocation Time"
