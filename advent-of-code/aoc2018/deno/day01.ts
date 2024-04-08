import { readArrayOfNumbers } from "./lib.ts";

export function part1(numbers: number[]) {
  const result = numbers.reduce((acc, val) => acc + val, 0);

  return result;
}

export function part2(numbers: number[]) {
  let current = 0;

  const seen = new Set();
  seen.add(current);

  while (true) {
    for (const n of numbers) {
      current += n;
      if (seen.has(current)) {
        return current;
      }
      seen.add(current);
    }
  }
}

if (import.meta.main) {
  const numbers = readArrayOfNumbers(1);
  console.log(part1(numbers));
  console.log(part2(numbers));
}
