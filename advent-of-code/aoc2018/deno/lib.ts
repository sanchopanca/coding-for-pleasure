export function readArrayOfNumbers(day: number) {
  console.assert(day > 0 && day <= 25);
  const padded = `${day}`.padStart(2, "0");
  const input = Deno.readTextFileSync(`../input/${padded}.txt`);
  const numbers = input
    .trim()
    .split("\n")
    .map((x) => parseInt(x));
  return numbers;
}

export function readArrayOfStrings(day: number) {
  console.assert(day > 0 && day <= 25);
  const padded = `${day}`.padStart(2, "0");
  const input = Deno.readTextFileSync(`../input/${padded}.txt`);
  return input
    .trim()
    .split("\n");
}
