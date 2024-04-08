export function part1(ids: string[]) {
  let [pairs, triplets] = [0, 0];
  for (const id of ids) {
    const counter: Map<string, number> = new Map();
    for (const c of id.split("")) {
      counter.set(c, (counter.get(c) ?? 0) + 1);
    }
    const values = [...counter.values()];
    if (values.includes(2)) {
      pairs++;
    }
    if (values.includes(3)) {
      triplets++;
    }
  }
  return pairs * triplets;
}

export function part2(ids: string[]) {
  for (const id1 of ids) {
    for (const id2 of ids) {
      if (hasOneDifference(id1, id2)) {
        return commonChars(id1, id2);
      }
    }
  }
}

function hasOneDifference(s1: string, s2: string) {
  console.assert(s1.length === s2.length);
  let d = 0;
  for (let i = 0; i < s1.length; i++) {
    if (s1[i] === s2[i]) {
      continue;
    }
    if (d) {
      return false;
    }
    d++;
  }

  return d === 1;
}

function commonChars(s1: string, s2: string) {
  console.assert(s1.length === s2.length);

  const result: string[] = [];
  for (let i = 0; i < s1.length; i++) {
    if (s1[i] === s2[i]) {
      result.push(s1[i]);
    }
  }
  return result.join("");
}

function readArrayOfStrings(day: number) {
  console.assert(day > 0 && day <= 25);
  const padded = `${day}`.padStart(2, "0");
  const input = Deno.readTextFileSync(`../input/${padded}.txt`);
  return input
    .trim()
    .split("\n");
}

if (import.meta.main) {
  const ids = readArrayOfStrings(2);
  console.log(part1(ids));
  console.log(part2(ids));
}
