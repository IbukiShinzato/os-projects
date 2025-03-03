set datafile separator comma
set terminal svg size 900,600
set output "data/read_macos/read_times.svg"
set title "File Size vs Time for Different Buffer Sizes"
set xlabel "File Size (KB)"
set ylabel "Time (ns)" 
set key outside
set style data lines
set grid
plot './data/read_macos/read_times.csv' using 2:3 every ::1::10 title "Buffer Size 0" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::11::20 title "Buffer Size 2" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::21::30 title "Buffer Size 4" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::31::40 title "Buffer Size 8" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::41::50 title "Buffer Size 16" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::51::60 title "Buffer Size 32" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::61::70 title "Buffer Size 64" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::71::80 title "Buffer Size 128" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::81::90 title "Buffer Size 256" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::91::100 title "Buffer Size 512" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::101::110 title "Buffer Size 1024" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::111::120 title "Buffer Size 2048" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::121::130 title "Buffer Size 4096" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::131::140 title "Buffer Size 8192" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::141::150 title "Buffer Size 16384" with lines, \
     './data/read_macos/read_times.csv' using 2:3 every ::151::160 title "Buffer Size 32768" with lines
