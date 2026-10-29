
install:
	chmod +x ./install.sh
	chmod +x ./trigger.sh
	./install.sh
	cargo build -r
	mv ./target/release/eery ~/.local/bin/eery
	echo "All Set."

default: install
