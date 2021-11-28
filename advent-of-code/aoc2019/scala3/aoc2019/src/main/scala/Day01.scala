import engineer.kovalev.aoc.Input

@main def day01: Unit =
  val x = Input.readListOfInts("../../input/01.txt")
  println(x.map(fuel).sum)
  println(x.map(fullFuel).sum)


def fuel(mass: Int): Int =
  mass / 3 - 2

def fullFuel(mass: Int): Int =
  var totalFuel = 0
  var toAdd = fuel(mass)
  while toAdd > 0 do
    totalFuel += toAdd
    toAdd = fuel(toAdd)
  totalFuel
        