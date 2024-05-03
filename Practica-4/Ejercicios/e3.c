#include <stdio.h>
#include <sys/wait.h>
#include <unistd.h>

int main() {
  int pid;
  printf("Hasta aqui hay un solo proceso...\n");
  // Creamos un nuevo proceso
  pid = fork();
  if (pid == 0) {
    printf("HIJO 1: Holaaa soy el primer hijo...\n");
    printf("HIJO 1: Voy a pararme por 5s y luego termino\n");
    sleep(20);
  } else if (pid > 0) {
    printf("PADRE: Hola, soy el padre. El PID de mi hijo es: %d\n", pid);
    pid = fork();

    if (pid == 0) {
      printf("HIJO 2: Holaaa, soy el segundo hijo...\n");
      printf("HIJO 2: Y voy a ejecutar la orden 'ls'...\n");

      execlp("ls", "ls", NULL);

      printf("Si ve este mensaje el execlp no funciono :(\n");
    } else if (pid > 0) {
      printf("PADRE: Hola otra vez. El PID de mi segundo hijo es: %d\n", pid);
      printf("PADRE: Voy a esperar a que terminen mis hijos...\n");
      printf("PADRE: Ha terminado mi hijo %d\n", wait(NULL));
      printf("PADRE: Ha terminado mi hijo %d\n", wait(NULL));
    }
  } else {
    printf("Hubo un error al llamar a fork()\n");
  }
}
