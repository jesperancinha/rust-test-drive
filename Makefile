install-mac:
	brew install cargo
	brew install rust
	brew install rustup
	cargo install grcov
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -y
install-rustup:
	rustup component add llvm-tools-preview
coverage:
	rustup override set nightly
	export RUSTFLAGS="-Zinstrument-coverage"
	#export RUSTC_WRAPPER=sccache
	cargo build
	cargo run
	cargo test
