#include <stdio.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

int main() {
  pid_t pid;
  pid = fork();

  if (pid < 0) {
    fprintf(stderr, "Fork Fallido");
    return 1;
  } else if (pid == 0) {
    execlp("/bin/ls", "ls", NULL);
  } else {
    wait(NULL);
    printf("Child completado\n");
  }

  return 0;
}
