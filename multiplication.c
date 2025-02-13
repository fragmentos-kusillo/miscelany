#include "api.h"
#include <stdio.h>


int whomai() {
    return 2;
}

int operation(int a, int b) {
    return a * b;
}

void load() {
    puts("Multiplication module loaded")
}

void unload() {
    puts("Multiplication module un-loaded")
}
