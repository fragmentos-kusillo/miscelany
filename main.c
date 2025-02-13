#include <stdio.h>
#include "api.h"

int main(void){
    int module_id = who_am_i();
    printf("Interacting with module: %d\n", module_id);

    load();
    int x = operation(4, 2);
    printf("Operation 4, 2: %d\n", x);
    unload();
    printf("\n\n");
}
