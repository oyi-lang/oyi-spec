.PHONY: all clean

all:
	peg -o phoron.c phoron.peg  && gcc -Wall -o phoron phoron.c main.c

clean:
	rm -f phoron phoron.c *.class