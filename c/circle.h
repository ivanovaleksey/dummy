#ifndef CIRCLE_H
#define CIRCLE_H

#include "point.h"

struct Circle {
  const struct Point *center;
  float radius;
};

float circle_area(const struct Circle *c);

#endif
