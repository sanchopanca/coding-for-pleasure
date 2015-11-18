module fmtgp.multiplication;

int multiplication1(uint n, uint a) {
  if (a == 1) {
    return n;
  }
  return multiplication1(n, a - 1) + a;
}
