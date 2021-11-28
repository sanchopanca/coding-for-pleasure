import engineer.kovalev.aoc.Input

@main def day04: Unit = 
  val rangeRaw = Input.readLine("../../input/04.txt")
  val range = rangeRaw.split("-").map(_.toInt)
  val (min, max) = (range(0), range(1))
  val validPasswords = (min to max).filter(isValidPassword)
  println(validPasswords.size)
  val validPasswords2 = (min to max).filter(isValidPassword2)
  println(validPasswords2.size)

def isValidPassword(password: Int): Boolean =
  val inOrder = !password.toString.toCharArray.sliding(2).exists(pair => pair(0) > pair(1))
  val hasDouble = password.toString.toCharArray.sliding(2).exists(pair => pair(0) == pair(1))
  inOrder && hasDouble

def isValidPassword2(password: Int): Boolean =
  val inOrder = !password.toString.toCharArray.sliding(2).exists(pair => pair(0) > pair(1))
  val hasDouble = password.toString.toCharArray.prepended('x').appended('x').sliding(4).exists(
    window => window(0) != window(1) && window(1) == window(2) && window(2) != window(3)
  )
  inOrder && hasDouble