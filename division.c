#include "api.h"
#include <stdio.h>


int who_am_i() {
    return 3;
}

int operation(int a, int b) {
    return a / b;
}

void load() {
    puts("Division module loaded");
}

void unload() {
    puts("Division module un-loaded");
}
