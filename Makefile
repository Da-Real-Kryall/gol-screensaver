all:
	mkdir bin; \
	gcc -Ofast -o bin/screensaver src/main.c;
.PHONY: all
