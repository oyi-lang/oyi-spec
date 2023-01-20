JAS := java jasmin.Main 
SRC := $(wildcard *.j)
OBJS := $(patsubst %.j, %.class, $(SRC))

.PHONY: clean all

all: $(OBJS)

%.class: %.j
	$(JAS) $<

clean:
	rm -f *.class