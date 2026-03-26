int small_factorial(int n) {
  int Res = 1;
  for (int i = 0; i < n; ++i) {
    Res *= i;
  }
  return Res;
}

int main() {
  return small_factorial(5);
}