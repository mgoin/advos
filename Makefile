NAME=advos
QEMU=qemu-system-riscv32
XARGO=xargo
TARGET=riscv32imac-unknown-none-elf
CROSS=riscv64-unknown-linux-gnu
CC=$(CROSS)-gcc
CXX=$(CROSS)-g++
AS=$(CROSS)-as
GDB=$(CROSS)-gdb

LDSFILE=lds/hifive.lds
ASFLAGS=-march=rv32ima -mabi=ilp32 -Og -g
LDFLAGS=-T$(LDSFILE) -march=rv32ima -mabi=ilp32 -Og -g -nostartfiles -nostdinc -ffreestanding -nostdlib -Ltarget/$(TARGET)/debug -L.
OUT=$(NAME).elf

QEMUARGS=-machine sifive_e -nographic -serial mon:stdio -kernel $(OUT)

ASM_SOURCES=$(wildcard asm/*.S)
ASM_OBJECTS=$(patsubst %.S,%.o,$(ASM_SOURCES))

RUST_SOURCES=$(wildcard src/*.rs)
RUST_OBJECT=target/$(TARGET)/debug/lib$(NAME).a

LIBS=-l$(NAME) -lgcc


all:FEATURES=--features hifive
all: $(OUT)

$(OUT): Makefile $(ASM_OBJECTS) $(RUST_OBJECT) $(LDSFILE)
	$(CC) $(LDFLAGS) -o $(OUT) $(ASM_OBJECTS) $(LIBS)

%.o: %.S Makefile
	$(CC) $(ASFLAGS) -c $< -o $@

$(RUST_OBJECT): Makefile $(RUST_SOURCES)
	$(XARGO) build --target=$(TARGET) $(FEATURES)

qemu: FEATURES=--features qemu
qemu: $(OUT)
	$(QEMU) $(QEMUARGS)

gdb: $(OUT)
	$(QEMU) $(QEMUARGS) -S -s &
	$(GDB) $(OUT) -ex "target remote localhost:1234"

.PHONY: clean

clean:
	$(XARGO) clean
	rm -fr $(OUT) $(ASM_OBJECTS)
