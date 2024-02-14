#include <stdio.h>


struct Name{
    char letter;
};

struct Address{
    struct Name *name;
    int zipcode;
    int houseNo;
};

int main(){
    int a = 6;
    int *p;
    p = malloc(sizeof(int));
    p = &a;
    printf("%d , %d, %p, %p\n", *p, a, p, &a);

    struct Address* addr;

    addr = malloc(sizeof(struct Address));
    addr->name = malloc(sizeof(struct Name));
    printf("%p, %p", addr->name, addr);
    struct Address **addr2;
    // *addr2 = 

}