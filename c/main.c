#include <stdio.h>
#include <stdlib.h>

#include "circle.h"
#include "point.h"

int main() {
  struct Point p = {
    .x = 0.14,
    .y = 2.1
  };

  struct Circle c = {
    .center = &p,
    .radius = 2
  };

  char str[25];

  point_info(&p, str, sizeof(str));
  printf("%s\n", str);

  printf("Circle's area = %f\n", circle_area(&c));

  return 0;
}
