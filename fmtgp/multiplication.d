module fmtgp.multiplication;


uint multiplication1(uint n, uint a) {
  if (a == 1) {
    return n;
  }
  return multiplication1(n, a - 1) + a;
}


uint egyptianMultiplication(uint n, uint a) {
  if (n == 1) {
    return a;
  }
  if (n % 2 == 0) {
    return egyptianMultiplication(n / 2, a + a);
  }
  return egyptianMultiplication((n - 1) / 2, a + a) + a;
}
