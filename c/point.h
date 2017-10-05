#ifndef POINT_H
#define POINT_H

struct Point {
  float x;
  float y;
};

int point_info(const struct Point *p, char *str, int size);

#endif
