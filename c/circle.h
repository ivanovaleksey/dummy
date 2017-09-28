#ifndef CIRCLE_H
#define CIRCLE_H

#include <math.h>
#include "point.h"

struct Circle {
  struct Point *center;
  float radius;
};

float circle_area(struct Circle *c);

#endif
