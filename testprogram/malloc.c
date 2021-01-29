#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char* argv[]) {
  assert(argc == 2);
  long size = atol(argv[1]);
  printf("Trying to allocate %d bytes\n", size);
  void* res = malloc(size);
  if (res == 0) {
    puts("Fail!");
    return 0;
  }
  puts("Using memory");
  memset(res, 0, size);
}
