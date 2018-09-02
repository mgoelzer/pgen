SRCS 	= src/pgen.rs

all:
	rustc -o pgen $(SRCS)
