build:
	cargo build --release -vv

run:
	cargo build --release
	./target/release/zx-qr-code -o ./qr-code.bin -u "https://zxwanderer.github.io/cell3326/1.2.1-aplha"
	./target/release/zx-qr-code -o ./qr-code.png -u "https://zxwanderer.github.io/cell3326/1.2.1-aplha"
