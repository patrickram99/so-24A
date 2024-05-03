#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <sys/mman.h>
#include <sys/shm.h>
#include <sys/stat.h>
#include <uchar.h>
#include <unistd.h>

int main() {
  const int SIZE = 4096;
  const char *name = "OS";
  const char *message_0 = "Hello";
  const char *message_1 = "World!";

  char *ptr;
  int fd;

  fd = shm_open(name, O_CREAT | O_RDWR, 0766);
  ftruncate(fd, SIZE);
  ptr = mmap(0, SIZE, PROT_WRITE, MAP_SHARED, fd, 0);
  sprintf(ptr, "%s", message_0);

  ptr += strlen(message_0);
  sprintf(ptr, "%s", message_1);
  strlen(message_1);
  return 0;
}
