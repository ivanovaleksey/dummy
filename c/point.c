#include "point.h"
#include <stdio.h>

int point_info(const struct Point *p, char *str, int size) {
  return snprintf(str, size, "x: %f,\ny: %f", p->x, p->y);
}
