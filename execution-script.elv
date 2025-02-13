#!/usr/local/bin/elvish

set E:LD_LIBRARY_PATH = (pwd)

gcc -c -Wall -Werror -fpic addition.c
gcc -shared -o lib0.so addition.c

gcc -c -Wall -Werror -fpic substraction.c
gcc -shared -o lib1.so substraction.o

gcc -c -Wall -Werror -fpic multiplication.c
gcc -shared -o lib2.so multiplication.o

gcc -c -Wall -Werror -fpic division.c
gcc -shared -o lib3.so division.o

echo "0 -> Addition\n1 -> Substraction\n2 -> Multiplication\n3 -> Division\n"

gcc -L(pwd) -Wall -o calculator main.c -l '0'; ./calculator
gcc -L(pwd) -Wall -o calculator main.c -l '1'; ./calculator
gcc -L(pwd) -Wall -o calculator main.c -l '2'; ./calculator
gcc -L(pwd) -Wall -o calculator main.c -l '3'; ./calculator

rm *.o *.so calculator
