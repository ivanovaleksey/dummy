#include "point.h"
#include <stdio.h>

void point_info(const struct Point *p, char *str) {
  sprintf(str, "x: %f,\ny: %f", p->x, p->y);
}
