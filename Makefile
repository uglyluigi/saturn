.DEFAULT_GOAL := build

build:
	cd threejspg/; /usr/bin/npm install; /usr/bin/npm run build; cd ..
	cp threejspg/dist/bg.js src/clientapp/src/resources/;	
	cd src/clientapp/; trunk build --release;
	cd src; cargo build --release;

testfront:
	cd threejspg/; /usr/bin/npm install; /usr/bin/npm run build; cd ..;
	cp threejspg/dist/bg.js src/clientapp/src/resources/; 
	cd src/clientapp/; trunk serve
