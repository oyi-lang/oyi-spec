.PHONY: all clean

all:
	make clean && peg -o phoron.c phoron.peg  && gcc -Wall -o phoron phoron.c main.c

clean:
	rm -f phoron