#include <stdio.h>

int main() {
  int res = 1;
  for (int i = 2; i < 5; ++i) {
    res *= i;
  }
  return res;
}