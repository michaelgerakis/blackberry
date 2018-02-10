# The target that we are compiling too, for xargo
TARGET := aarch64-none-elf
# Variables for the target build commands
XARGO := CARGO_INCREMENTAL=0 RUST_TARGET_PATH="$(shell pwd)" xargo
CC      := $(TARGET)-gcc
LD      := $(TARGET)-ld
OBJCOPY := $(TARGET)-objcopy

CCFLAGS := -Wall -O2 -nostdlib -nostartfiles -ffreestanding -pie -fpie
LDFLAGS := --gc-sections -static -nostdlib -nostartfiles --no-dynamic-linker

BINARY_NAME := blackberry
BUILD_DIR := build

# Rust files are build into this directory
RUST_BUILD_DIR := target/$(TARGET)
RUST_DEBUG_LIB := $(RUST_BUILD_DIR)/debug/lib$(BINARY_NAME).a
RUST_RELEASE_LIB := $(RUST_BUILD_DIR)/release/lib$(BINARY_NAME).a

KERNEL := $(BUILD_DIR)/$(BINARY_NAME)

# Search this path if the file is not in the current directory
VPATH := ext

RUST_DEPS := $(wildcard src/*.rs) $(wildcard volatile/src/*.rs)

.PHONY: all clean install format deps check

all: $(KERNEL).bin $(KERNEL).hex

$(KERNEL).bin: $(KERNEL).elf
	@echo 'Creating binary output.'
	@$(OBJCOPY) $< -O binary $@

$(KERNEL).hex: $(KERNEL).elf
	@echo 'Creating hex output.'
	@$(OBJCOPY) $< -O ihex $@

$(KERNEL).elf: $(BUILD_DIR)/boot.o $(RUST_DEBUG_LIB)
	@echo 'Linking the files'
	@$(LD) $(LDFLAGS) -T ext/layout.ld -o $@ -O2 $^

$(BUILD_DIR):
	@mkdir $@

$(RUST_DEBUG_LIB): $(RUST_DEPS)
	@echo 'Building kernel.'
	@$(XARGO) build --target $(TARGET)

$(BUILD_DIR)/%.o: %.S | $(BUILD_DIR)
	@echo 'Creating boot.S.'
	@$(CC) $(CCFLAGS) -c $< -o $@

check: $(RUST_DEPS)
	@$(XARGO) check --target $(TARGET)

deps:
	rustup component add rustfmt-preview rust-src

install: $(KERNEL).bin
	@echo 'Installing disk onto usb drive...'
	@if [ -d "/Volumes/CANAKIT" ]; then mv $< /Volumes/CANAKIT/kernel8.img; fi
	@echo 'Unmounting usb drive...'
# Sleep for a second to make sure everything is written
	@sleep 1
	@diskutil unmount CANAKIT

format: $(RUST_DEPS)
	@rustfmt $?

clean:
	@echo 'Removing remnants...'
	@$(XARGO) clean
	@rm -rf $(BUILD_DIR)
	@echo 'Done.'
