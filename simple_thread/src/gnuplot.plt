#!/usr/bin/gnuplot
set title "{/=20 Nanosleep delays on Linux 3.6 Kernels}"
set terminal png
set xlabel "{/=20Period to sleep [microseconds]}\\n"
set ylabel "{/=20Measured delay [microseconds]}"
set output 'output.png' 
set grid
set autoscale
set style data lines
show style data
 plot 'output' using ($1):($2) title "{/=17 no Load no RT: Linux 3.6}" lw 6
