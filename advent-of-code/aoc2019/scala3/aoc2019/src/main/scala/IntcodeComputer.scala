package engineer.kovalev.aoc.intcode

import scala.io.StdIn.readInt

enum Mode:
    case Position
    case Immediate

class IntcodeComputer(var memory: Array[Int]):
  val ADD = 1
  val MULTIPLY = 2
  val INPUT = 3
  val OUTPUT = 4
  val JUMP_IF_TRUE = 5
  val JUMP_IF_FALSE = 6
  val LESS_THAN = 7
  val EQUALS = 8
  val HALT = 99

  private def input(): Int =
    print("Input: ")
    readInt
  
  private def output(value: Int): Unit =
    println(value)

  private var pointer: Int = 0
  def execute() =
    var (opCode, modes) = parseOpCode(memory(pointer))
    while opCode != HALT do
      opCode match
        case ADD =>
          val op1 = if modes(0) == Mode.Position then memory(pointer + 1) else pointer + 1
          val op2 = if modes(1) == Mode.Position then memory(pointer + 2) else pointer + 2
          val dest = memory(pointer + 3)
          memory(dest) = memory(op1) + memory(op2)
          if dest != pointer then pointer += 4
        case MULTIPLY =>
          val op1 = if modes(0) == Mode.Position then memory(pointer + 1) else pointer + 1
          val op2 = if modes(1) == Mode.Position then memory(pointer + 2) else pointer + 2
          val dest = memory(pointer + 3)
          memory(dest) = memory(op1) * memory(op2)
          if dest != pointer then pointer += 4
        case INPUT =>
          val dest = memory(pointer + 1)
          memory(dest) = input()
          if dest != pointer then pointer += 2
        case OUTPUT =>
          val op1 = if modes(0) == Mode.Position then memory(pointer + 1) else pointer + 1
          output(memory(op1))
          pointer += 2
        case JUMP_IF_TRUE =>
          val op1 = if modes(0) == Mode.Position then memory(pointer + 1) else pointer + 1
          val op2 = if modes(1) == Mode.Position then memory(pointer + 2) else pointer + 2
          if memory(op1) != 0 then
            pointer = memory(op2)
          else
            pointer += 3
        case JUMP_IF_FALSE =>
          val op1 = if modes(0) == Mode.Position then memory(pointer + 1) else pointer + 1
          val op2 = if modes(1) == Mode.Position then memory(pointer + 2) else pointer + 2
          if memory(op1) == 0 then
            pointer = memory(op2)
          else
            pointer += 3
        case LESS_THAN =>
          val op1 = if modes(0) == Mode.Position then memory(pointer + 1) else pointer + 1
          val op2 = if modes(1) == Mode.Position then memory(pointer + 2) else pointer + 2
          val dest = memory(pointer + 3)
          memory(dest) = if memory(op1) < memory(op2) then 1 else 0
          if dest != pointer then pointer += 4
        case EQUALS =>
          val op1 = if modes(0) == Mode.Position then memory(pointer + 1) else pointer + 1
          val op2 = if modes(1) == Mode.Position then memory(pointer + 2) else pointer + 2
          val dest = memory(pointer + 3)
          memory(dest) = if memory(op1) == memory(op2) then 1 else 0
          if dest != pointer then pointer += 4
        case _ => throw new IllegalArgumentException(s"Unknown opcode: $opCode")
        val tmpTuple = parseOpCode(memory(pointer))
        opCode = tmpTuple._1
        modes = tmpTuple._2
  
  def parseOpCode(opCode: Int): (Int, Array[Mode]) =
    val code = opCode % 100
    val mode1 = if (opCode / 100 % 10 == 0) Mode.Position else Mode.Immediate
    val mode2 = if (opCode / 1000 % 10 == 0) Mode.Position else Mode.Immediate
    val mode3 = if (opCode / 1000 == 0) Mode.Position else Mode.Immediate
    (code, Array(mode1, mode2, mode3))


