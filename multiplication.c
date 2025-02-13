#include "api.h"
#include <stdio.h>


int who_am_i() {
    return 2;
}

int operation(int a, int b) {
    return a * b;
}

void load() {
    puts("Multiplication module loaded");
}

void unload() {
    puts("Multiplication module un-loaded");
}
