.DEFAULT_GOAL := build

build:
	cd threejspg/; npm run build; cd ..
	cp threejspg/dist/bg.js src/clientapp/src/assets/	
	cd src/clientapp/; trunk build --release;
	cd src; cargo build --release;
