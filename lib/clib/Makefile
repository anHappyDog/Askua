include ../../include.mk

INCLUDES := -Iinclude

CLIB := libclib.a


sources := $(wildcard *.c) $(wildcard *.S)
objects := $(patsubst %.c,%.o,$(sources))
objects := $(patsubst %.S,%.o,$(objects))

%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDES) -c -o $@ $<

%.o: %.S
	$(CC) $(CFLAGS) $(INCLUDES) -c -o $@ $<

.PHONY: clean

all: $(CLIB)

$(CLIB): $(objects)
	mkdir -p ../$(CLIB_DIR)
	$(AR) rcs ../$(CLIB_DIR)/$@ $^

clean:
	rm -rf *.o *.a
