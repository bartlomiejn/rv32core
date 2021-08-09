ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

EXTERNAL_DIR := $(ROOT_DIR)/external
RISCV_TESTS_DIR := $(EXTERNAL_DIR)/riscv-tests
OUTPUT_DIR := $(ROOT_DIR)/OUTPUT_DIR

tests:
	cd $(RISCV_TESTS_DIR) \
		&& autoconf \
		&& ./configure --prefix=$$RISCV/target \
		&& make
