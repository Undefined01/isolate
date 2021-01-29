#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void payload(char* self) { execlp("bash", "bash", "-c", "whoami"); }

int main(int argc, char* argv[]) {
  assert(argc == 2);
  long count = atol(argv[1]);
  printf("Trying to create %d process\n", count);
  for (int i = 0; i < count; i++) {
    int res = fork();
    if (res < 0) {
      puts("Failed to fork!");
      return 1;
    }
    if (res == 0) payload(argv[0]);
  }
  while (1)
    ;
}
