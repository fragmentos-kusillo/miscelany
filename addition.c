#include "api.h"
#include <stdio.h>


int who_am_i() {
    return 0;
}

int operation(int a, int b) {
    return a + b;
}

void load() {
    puts("Addition module loaded");
}

void unload() {
    puts("Addition module un-loaded");
}
