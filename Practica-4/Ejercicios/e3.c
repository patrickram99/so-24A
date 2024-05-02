#include <stdio.h>
#include <unistd.h>

int main(void) {
  int pid;
  printf("Hasta aqui hay un unico proceso...\n");
  printf("Primera llamada a fork...\n");

  pid = fork();

  if (pid == 0) {
    printf("HIJO 1: Holaaa I'm the first son...\n");

    printf("HIJO 1: Voy a descansar 5 segundos y luego termino\n");
    sleep(20);
  } else if (pid > 0) {
    printf("PADRE: Hola, soy el padre. El PID de mi hijo es: %d\n", pid);
    pid = fork();

    if (pid == 0) {
      printf("HIJO 2: Holaaa I'm the second son...\n");
      printf("HIJO 2: ");
    }
  }
}
