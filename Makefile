.PHONY: all
all: build

.PHONY: build
build:
	cargo build

.PHONY: docs
docs:
	cargo doc

.PHONY: test
test:
	cargo test

TARGET_DIR := target/debug
COVERAGE_DIR := $(TARGET_DIR)/coverage
PROFRAW := $(TARGET_DIR)/coverage.profraw
LCOV := $(TARGET_DIR)/lcov.info

.PHONY: test_with_cov
test_with_cov: 
	RUSTFLAGS="-C instrument-coverage=all" \
		LLVM_PROFILE_FILE="$(PROFRAW)" cargo test
	grcov . -s . --binary-path $(TARGET_DIR) -t lcov --branch \
		--ignore-not-existing --ignore "/*" --ignore "target/*" \
		-o $(LCOV)
	genhtml -o $(COVERAGE_DIR) --show-details --ignore-errors source \
		--legend $(LCOV) -rc genhtml_dark_mode=1

.PHONY: gen_info
gen_info: docs test_with_cov

BINARY := $(TARGET_DIR)/o2c
MEMCHECK_BINARY_ARGS := --display-tokens ./examples/basic.o2

.PHONY: memcheck
memcheck: build
	valgrind --leak-check=full --show-leak-kinds=all -s $(BINARY) $(MEMCHECK_BINARY_ARGS)

.PHONY: clean
clean:
	cargo clean
