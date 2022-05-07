all:
	cd ./bin; \
	gcc -Ofast -o screensaver ../src/main.c; \
	./screensaver;
.PHONY: all