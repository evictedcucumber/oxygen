.PHONY: test-cov test-build http

all: test-cov

test-build:
	RUSTFLAGS="-C instrument-coverage=all" LLVM_PROFILE_FILE="coverage/default.profraw" cargo test

test-cov: test-build
	cargo profdata -- merge -sparse coverage/default.profraw -o coverage/default.profdata
	PATH=$$PATH:$$HOME/.cargo/bin cargo cov -- show --Xdemangler=rustfilt --ignore-filename-regex='./cargo/' --instr-profile=coverage/default.profdata \
		--format=html --output-dir=coverage/html $(shell find target/debug/deps -type f -name "o2c*" -not -name "*.d")
