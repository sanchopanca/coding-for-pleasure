import engineer.kovalev.aoc.Input
import engineer.kovalev.aoc.intcode.IntcodeComputer

@main def day02: Unit =
  println(calculate(12, 2))
  for noun <- 0 to 99 do
    for verb <- 0 to 99 do
      if calculate(noun, verb) == 19690720 then
        println(100 * noun + verb)
        return

def calculate(noun: Int, verb: Int): Int =
  val intcode = Input.readCommaSeparatedInts("../../input/02.txt").toArray
  intcode(1) = noun
  intcode(2) = verb
  val computer = IntcodeComputer(intcode)
  computer.execute()
  computer.memory(0)