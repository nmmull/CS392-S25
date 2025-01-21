#include<stdio.h>

char* s1 =  "hello";
char* s2 =  "world";

void swap(char **x, char **y) {
  char *z = *x;
  *x = *y;
  *y = z;
}

int main(void) {
  swap(&s1, &s2);
  printf("%s\n", s1);
  return 0;
}
