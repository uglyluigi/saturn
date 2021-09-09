.DEFAULT_GOAL := build

build:
	cd threejspg/; npm run build;
	cp threejspg/dist/bg.js /src/clientapp/;
	cd src/clientapp/; trunk build --release;
	cd src; cargo build --release;