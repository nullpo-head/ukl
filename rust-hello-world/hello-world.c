#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int
main(int argc, char *argv[])
{
  printf("my hello world\n");
  for (;;)
    pause ();
  exit(EXIT_SUCCESS);
}
