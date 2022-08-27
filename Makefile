.PHONY : clib
clib:
	cd c_wrapper
	make

.PHONY: build
build: 
	clib
	cargo b

.PHONY : valgrind
valgrind:
	cd target/debug
	valgrind ./ogdf-rs
	cd ../..

.PHONY : leek
leek:
	build
	valgrind