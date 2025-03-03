set datafile separator comma
set terminal svg size 900,600
set output './img/cpu30_results.svg'
set xlabel "Parallelization Ratio"
set ylabel "Effect of Paralleling"
set xrange [0:100] 
set xtics 10
set style data lines
set grid
set key outside

plot './data/cpu30_results.csv' using 1:4 every ::1::11 title "1 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::12::22 title "2 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::23::33 title "3 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::34::44 title "4 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::45::55 title "5 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::56::66 title "6 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::67::77 title "7 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::78::88 title "8 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::89::99 title "9 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::100::110 title "10 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::111::121 title "11 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::122::132 title "12 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::133::143 title "13 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::144::154 title "14 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::155::165 title "15 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::166::176 title "16 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::177::187 title "17 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::188::198 title "18 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::199::209 title "19 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::210::220 title "20 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::221::231 title "21 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::232::242 title "22 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::243::253 title "23 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::254::264 title "24 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::265::275 title "25 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::276::286 title "26 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::287::297 title "27 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::298::308 title "28 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::309::319 title "29 cpu" with lines lw 2, \
     './data/cpu30_results.csv' using 1:4 every ::320::330 title "30 cpu" with lines lw 2, \
     
