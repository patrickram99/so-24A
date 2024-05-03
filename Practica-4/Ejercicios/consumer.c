#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>

#include <sys/mman.h>
#include <sys/shm.h>
#include <sys/stat.h>
#include <unistd.h>

int main() {
  const int SIZE = 4096;
  const char *name = "OS";
  int fd;
  char *ptr;

  fd = shm_open(name, O_RDONLY, 0766);
  ptr = mmap(0, SIZE, PROT_READ, MAP_SHARED, fd, 0);

  printf("%s", (char *)ptr);
  shm_unlink(name);

  return 0;
}
