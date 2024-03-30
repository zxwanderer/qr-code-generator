build:
	cargo build --release

run:
	cargo build --release
	./target/release/zx-qr-code -o ./qr-code.png -u "https://zxwanderer.github.io/cell3326/1.2.1-aplha" -s 32
