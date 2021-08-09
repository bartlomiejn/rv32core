ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

EXT_DIR := $(ROOT_DIR)/external
EXT_TESTS_DIR := $(EXT_DIR)/riscv-tests
OUTPUT_DIR := $(ROOT_DIR)/output
TESTS_DIR := $(OUTPUT_DIR)/riscv-tests

$(OUTPUT_DIR)/.sync-tests: $(EXT_TESTS_DIR)
	mkdir -p $(TESTS_DIR)
	rsync --exclude='.git' -al $</ $(TESTS_DIR)
	touch $@

tests: $(OUTPUT_DIR)/.sync-tests
	cd $(TESTS_DIR) \
		&& autoconf \
		&& ./configure \
		&& make

clean:
	rm -rf $(TESTS_DIR)