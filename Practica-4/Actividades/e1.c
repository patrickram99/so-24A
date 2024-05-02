#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(void) {
  printf("Proceso hijo PID = %d\n", getpid());
  printf("Proceso padre PID = %d\n", getppid());

  return EXIT_SUCCESS;
}
