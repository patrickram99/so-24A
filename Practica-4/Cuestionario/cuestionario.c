#include <stdio.h>
#include <sys/wait.h>
#include <unistd.h>

int main() {
    int pid;
    
    pid = fork();
    if (pid == 0) {
        printf("Hola ");
        pid = fork();
        if (pid == 0) {
            printf("Buenos ");
        } else {
            wait(NULL);
            pid = fork();
            if (pid == 0) {
                printf("dias ");
            } else {
                wait(NULL);
                pid = fork();
                if (pid == 0) {
                    printf("tenga ");
                } else {
                    wait(NULL);
                    pid = fork();
                    if (pid == 0) {
                        printf("usted. ");
                    } else {
                        wait(NULL);
                        printf("Hasta luego Lucas\n");
                    }
                }
            }
        }
    } else {
        wait(NULL);
    }
    return 0;
}