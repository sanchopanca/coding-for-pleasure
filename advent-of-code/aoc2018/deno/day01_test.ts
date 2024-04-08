import { assertEquals } from "jsr:@std/assert";
import { part1, part2 } from "./day01.ts";

Deno.test(function testPart1() {
  assertEquals(part1([1, -1]), 0);
  assertEquals(part1([1, 1, -1]), 1);
});

Deno.test(function testPart2() {
  assertEquals(part2([1, 1, -1]), 1);
  assertEquals(part2([3, 3, 4, -2, -4]), 10);
});
