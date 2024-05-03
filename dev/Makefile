INCLUDES    := -I../include/

sources := $(wildcard *.c) $(wildcard *.S)
objects := $(patsubst %.c,%.o,$(sources))
objects := $(patsubst %.S,%.o,$(objects))

%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDES) -c -o $@ $<

%.o: %.S
	$(CC) $(CFLAGS) $(INCLUDES) -c -o $@ $<

.PHONY: clean

all: $(objects)

clean:
	rm -rf *.o