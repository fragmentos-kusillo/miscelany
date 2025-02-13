#include "api.h"
#include <stdio.h>


int who_am_i() {
    return 1;
}

int operation(int a, int b) {
    return a + b;
}

void load() {
    puts("Substraction module loaded");
}

void unload() {
    puts("Substraction module un-loaded");
}
