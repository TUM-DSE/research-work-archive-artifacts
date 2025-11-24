.phony: all clean cleanup
all: CPU_IPC.pdf latency_rtime.pdf latency_hotspot.pdf overall_rtime.pdf SLC_latency.pdf SLC_rtime.pdf SLC_rtime_CPU.pdf TCC_Hitrate.pdf cleanup	

cleanup: 
	rm -f *.aux *.log
clean:
	rm -f *.aux *.log *.out *.pdf 

%.pdf: %.tex
	pdflatex $<

