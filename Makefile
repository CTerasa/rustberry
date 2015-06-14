
# rustc target
TARGET = arm-unknown-linux-gnueabihf

# toolchain prefix
TRIPLE = arm-none-eabi

# taken from http://stackoverflow.com/a/12959694
rwildcard=$(wildcard $(addsuffix $2, $1)) $(foreach d,$(wildcard $(addsuffix *, $1)),$(call rwildcard,$d/,$2))

DEPS_DIR = $(OUT_DIR)/deps
OUT_DIR = images

BINS = $(OUT_DIR)/kernel.img
ELFS = $(OUT_DIR)/kernel.elf

RSRC := src/lib.rs
ASRC := src/arch/arm/mach_bcm2835/bootstrap.s
ROBJ := $(RSRC:.rs=.o)
AOBJ := $(ASRC:.s=.o)

RUSTC_FLAGS := -O \
	       -Z no-landing-pads \
	       -C no-stack-check \
	       -C relocation-model=static \
	       $(RUSTC_FLAGS)

# don't delete my elf files!
.SECONDARY:

#all: rlibs  $(APPS)
all:  $(BINS)

clean:
	rm -f $(AOBJ) $(ROBJ) $(BINS) $(ELFS)

%.o: %.rs
	rustc \
		$(RUSTC_FLAGS) \
		--crate-type rlib \
		--emit obj \
		--target $(TARGET) \
		-L $(DEPS_DIR) \
		--verbose \
		-o $@ \
		$<

%.o: %.s
	$(TRIPLE)-as \
		--verbose \
		-o $@ \
		$<

$(ELFS): $(ROBJ) $(AOBJ)
	mkdir -p $(dir $@)
	$(TRIPLE)-ld \
	-T layout.ld \
	-o $@ \
	$^

$(BINS): $(ELFS)
	mkdir -p $(dir $@) 
	$(TRIPLE)-objcopy \
		-O binary \
		$< \
		$@
