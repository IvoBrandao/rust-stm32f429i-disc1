TARGET = thumbv7em-none-eabihf
BIN    = target/$(TARGET)/debug/stm32f429i-disc1
GDB    = gdb

.PHONY: all clean build flash debug

all: build

clean:
	cargo clean

build:
	cargo build

flash: build
	openocd -f openocd.cfg -c "program $(BIN) verify reset exit"

debug: clean build
	openocd -f openocd.cfg &
	$(GDB) -q -x openocd.gdb $(BIN); \
	kill %1 2>/dev/null; true
