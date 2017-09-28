#include "circle.h"

float circle_area(struct Circle *c) {
  return pow(c->radius, 2) * M_PI;
}
