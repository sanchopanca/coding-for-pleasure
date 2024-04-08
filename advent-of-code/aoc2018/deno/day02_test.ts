import { assertEquals } from "jsr:@std/assert";
import { part1, part2 } from "./day02.ts";

const ids1 = [
  "abcdef",
  "bababc",
  "abbcde",
  "abcccd",
  "aabcdd",
  "abcdee",
  "ababab",
];

const ids2 = [
  "abcde",
  "fghij",
  "klmno",
  "pqrst",
  "fguij",
  "axcye",
  "wvxyz",
];

Deno.test(function testPart1() {
  assertEquals(part1(ids1), 12);
});

Deno.test(function testPart2() {
  assertEquals(part2(ids2), "fgij");
});
