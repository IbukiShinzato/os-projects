set datafile separator comma
set terminal svg size 900,600
set output './img/cpu4_results.svg'
set xlabel "Parallelization Ratio"
set ylabel "Effect of Paralleling"
set xrange [0:100] 
set xtics 10
set style data lines
set grid
set key outside

plot './data/cpu4_results.csv' using 1:4 every ::1::11 title "1 cpu" with lines lw 2, \
     './data/cpu4_results.csv' using 1:4 every ::12::22 title "2 cpu" with lines lw 2, \
     './data/cpu4_results.csv' using 1:4 every ::23::33 title "3 cpu" with lines lw 2, \
     './data/cpu4_results.csv' using 1:4 every ::34::44 title "4 cpu" with lines lw 2
