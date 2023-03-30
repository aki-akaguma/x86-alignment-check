
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test --offline
	cargo test --offline -- --ignored 2>&1 | grep "(signal: 7, SIGBUS: access to undefined memory)"
	@echo OK

test-no-default-features:
	cargo test --offline --no-default-features

miri:
	cargo +nightly miri test --offline

clean:
	@cargo clean
	@rm -f z.*

clippy:
	cargo clippy --offline --tests --workspace -- -W clippy::uninlined_format_args

fmt:
	cargo fmt

doc:
	cargo doc
