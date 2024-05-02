#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>

int crear(char *programa, char **argumentos) {
  pid_t pid_hijo;
  pid_hijo = fork();

  if (pid_hijo != 0) {
    sleep(20);
    return pid_hijo;
  } else {
    execv(programa, argumentos);
    fprintf(stderr, "Se ha generado un error");
    abort();
  }
}

int main() {
  char *argumentos[] = {"ls", "-lh", "/", NULL};
  crear("/bin/ls", argumentos);

  return EXIT_SUCCESS;
}
