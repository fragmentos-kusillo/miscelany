#include "api.h"
#include <stdio.h>


int whomai() {
    return 1;
}

int operation(int a, int b) {
    return a + b;
}

void load() {
    puts("Substraction module loaded")
}

void unload() {
    puts("Substraction module un-loaded")
}
