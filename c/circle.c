#include "circle.h"
#include <math.h>

float circle_area(struct Circle *c) {
  return pow(c->radius, 2) * M_PI;
}
