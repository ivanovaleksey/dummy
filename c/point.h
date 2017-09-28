#ifndef POINT_H
#define POINT_H

struct Point {
  float x;
  float y;
};

void *point_info(struct Point *p, char *str);

#endif
