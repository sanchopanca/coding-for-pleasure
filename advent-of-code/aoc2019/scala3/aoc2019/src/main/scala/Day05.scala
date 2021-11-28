import engineer.kovalev.aoc.Input
import engineer.kovalev.aoc.intcode.IntcodeComputer

@main def day05: Unit =
  val program = Input.readCommaSeparatedInts("../../input/05.txt")
  var c = IntcodeComputer(program.toArray)
  c.execute()
  c = IntcodeComputer(program.toArray)
  c.execute()