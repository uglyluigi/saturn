.DEFAULT_GOAL := build

build:
	cd src/clientapp/; trunk build --release;
	cd src; cargo build --release;