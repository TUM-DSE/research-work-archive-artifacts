#include <stdio.h>

int factorial(int n) {
  if (n < 1) {
    return 1;
  }
  return n * factorial(n - 1);
}

int main() {
  int a = factorial(5);
  putchar(a);
  return a;
}