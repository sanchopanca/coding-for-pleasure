import { readArrayOfStrings } from "./lib.ts";

export function part1(claims: Claim[]) {
  const fabric = init2DArray(2000, 2000, 0);

  for (const { rect: { x, y, w, h } } of claims) {
    for (let j = y; j < y + h; j++) {
      for (let i = x; i < x + w; i++) {
        fabric[j][i]++;
      }
    }
  }

  let overlapped = 0;
  for (let i = 0; i < fabric.length; i++) {
    for (let j = 0; j < fabric[i].length; j++) {
      if (fabric[i][j] > 1) {
        overlapped++;
      }
    }
  }
  return overlapped;
}

function init2DArray(rows: number, cols: number, value: number): number[][] {
  return Array.from({ length: rows }, () => Array(cols).fill(value));
}

export function part2(claims: Claim[]) {
  const fabric = init2DArray(2000, 2000, 0);
  const cleanIds: Set<number> = new Set();

  for (const { id, rect: { x, y, w, h } } of claims) {
    let overlapped = false;
    for (let j = y; j < y + h; j++) {
      for (let i = x; i < x + w; i++) {
        if (fabric[j][i]) {
          overlapped = true;
          cleanIds.delete(fabric[j][i]);
        }
        fabric[j][i] = id;
      }
    }
    if (!overlapped) {
      cleanIds.add(id);
    }
  }

  console.assert(cleanIds.size === 1);
  return cleanIds.values().next().value;
}

interface Rectangle {
  x: number;
  y: number;
  w: number;
  h: number;
}

interface Claim {
  id: number;
  rect: Rectangle;
}

if (import.meta.main) {
  const input = readArrayOfStrings(3);
  const claims: Claim[] = [];

  for (const line of input) {
    const [idStr, rest] = line.split(" @ ");
    const id = parseInt(idStr.replace("#", ""));
    const [corner, dim] = rest.split(": ");
    const [x, y] = corner.split(",").map((x) => parseInt(x));
    const [w, h] = dim.split("x").map((x) => parseInt(x));

    claims.push({
      id,
      rect: { x, y, w, h },
    });
  }
  console.log(part1(claims));
  console.log(part2(claims));
}
