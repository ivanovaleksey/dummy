#include "point.h"
#include <stdio.h>

void *point_info(struct Point *p, char *str) {
  sprintf(str, "x: %f,\ny: %f", p->x, p->y);
  return str;
}
