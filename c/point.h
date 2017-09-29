#ifndef POINT_H
#define POINT_H

struct Point {
  float x;
  float y;
};

void point_info(const struct Point *p, char *str);

#endif
