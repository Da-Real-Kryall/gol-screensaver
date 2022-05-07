# Screensaver Thingy

This repo contains the source of a screensaver I have on my laptop; just the executable that prints the pretty patterns, not the programs that launch it on inactivity; those are highly platform specific and frankly i have no idea how to implement them here.

---

## Instructions for building:

1. Download and unzip the repository files.
2. CD into the project directory:
    
    ```bash
    cd /path/to/gol-screensaver/
    ```

3. Run the make command to execute the makefile.
    ```bash
    make
    ```

The compiled binary should be in the `/bin` directory as "screensaver", though after running `make` the executable should run automatically.

---

## Other info:

This was programmed in C as one of my first largeish projects using the language. I also did a rewrite a few months after its initial post to github as some practice with using header files among other things.

If you are curious as to how I have made this a screensaver on MacOS; I used a piece of software called "Sleepwatcher" to trigger executable files and bash scripts on certain inactivity related events.
