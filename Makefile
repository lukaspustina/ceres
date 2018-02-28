all: check build test

build-lib:
	cargo build

build:
	cargo build

check:
	cargo check

test:
	cargo test

use_case_tests: use_cases
	make -C $<

docs: doctoc man
	
doctoc: README.md
	doctoc $<

man:
	$(MAKE) -C docs

clippy:
	rustup run nightly cargo clippy

fmt:
	rustup run nightly cargo fmt

duplicate_libs:
	cargo tree -d

_update-clippy_n_fmt:
	rustup update
	rustup run nightly cargo install clippy --force
	rustup component add rustfmt-preview --toolchain=nightly

