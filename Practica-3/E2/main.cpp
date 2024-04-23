#include <stdio.h>
#include <string.h>
#include <unistd.h> 

int main(int argc, char *argv[])
{
    int segundos;
    if (argc != 3) {
        fprintf(stderr, "Uso: %s <segundos> <mensaje>\n", argv[0]);
        return 1;
    }
    sscanf(argv[1], "%d", &segundos);
    while (1) { 
        sleep(segundos);
        printf("%s\n", argv[2]);
    }
    return 0;
}
