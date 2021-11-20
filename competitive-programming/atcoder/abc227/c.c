#include <stdio.h>
#include <math.h>

int main()
{
  long long n;
  scanf("%lld", &n);
  long long result = 0;
  for (long long a = 1; a <= (long long)floor(pow(n, 1.0/3.0)); ++a) {
    for (long long b = a; b <= (long long)floor(sqrt((double)n/a)); ++b) {
      result += (long long)floor((double)n/(a*b)) - b + 1;
    }
  }
  printf("%lld\n",result);
  return 0;
}