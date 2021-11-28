package engineer.kovalev.aoc.Input

import scala.io.Source

def readListOfInts(filename: String): List[Int] =
  Source.fromFile(filename).getLines.toList.map(_.toInt)

def readCommaSeparatedInts(filename: String): List[Int] =
  Source.fromFile(filename).getLines.toList.flatMap(_.split(",").map(_.toInt))

def readCommaSeparatedStrings(filename: String): List[List[String]] =
  Source.fromFile(filename).getLines.toList.map(_.split(",").toList)

def readLine(filename: String): String =
  Source.fromFile(filename).getLines.toList.head
