.PHONY: test_coverage clean all

TARGET_DIR := target/debug
COVERAGE_DIR := $(TARGET_DIR)/coverage
PROFRAW := $(TARGET_DIR)/coverage.profraw
LCOV := $(TARGET_DIR)/lcov

all: test_coverage

clean:
	cargo clean

test_coverage:
	RUSTFLAGS="-C instrument-coverage=all" LLVM_PROFILE_FILE="$(PROFRAW)" cargo test
	grcov . -s . --binary-path $(TARGET_DIR) -t lcov --branch --ignore-not-existing -o $(TARGET_DIR)
	genhtml -o $(COVERAGE_DIR) --show-details --ignore-errors source --legend $(LCOV)

