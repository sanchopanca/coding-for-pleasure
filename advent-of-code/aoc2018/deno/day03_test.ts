import { assertEquals } from "jsr:@std/assert";
import { part1, part2 } from "./day03.ts";

const claims = [
  {
    id: 1,
    rect: {
      x: 1,
      y: 3,
      w: 4,
      h: 4,
    },
  },
  {
    id: 2,
    rect: {
      x: 3,
      y: 1,
      w: 4,
      h: 4,
    },
  },
  {
    id: 3,
    rect: {
      x: 5,
      y: 5,
      w: 2,
      h: 2,
    },
  },
];

Deno.test(function testPart1() {
  assertEquals(part1(claims), 4);
});

Deno.test(function testPart2() {
  assertEquals(part2(claims), 3);
});
