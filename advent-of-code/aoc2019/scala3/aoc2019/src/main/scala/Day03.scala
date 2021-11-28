import scala.collection.mutable.Set
import scala.collection.mutable.Map

import engineer.kovalev.aoc.Input

@main def day03: Unit = 
  val instructions = Input.readCommaSeparatedStrings("../../input/03.txt")
  
  val firstPath = everyCoordinate(instructions(0))
  val secondPath = everyCoordinate(instructions(1))
  val intersections = firstPath & secondPath
  val min = intersections.map(c => c._1.abs + c._2.abs).min
  println(min)

  val firstPathLengths = coordinatesToPathLength(instructions(0))
  val secondPathLengths = coordinatesToPathLength(instructions(1))
  val intersections2 = firstPathLengths.keySet & secondPathLengths.keySet
  val min2 = intersections2.map(c => firstPathLengths(c) + secondPathLengths(c)).min
  println(min2)

def coordinatesToPathLength(instructions: List[String]): Map[(Int, Int), Int] =
  val pathLength = Map[(Int, Int), Int]()
  var current = (0, 0)
  var curLength = 0
  instructions.foreach(instruction =>
    val r = raw"(U|D|L|R)(\d+)".r
    instruction match
      case r(direction, distance) =>
        // println(s"$direction $distance")
        direction match
          case "U" =>
            for i <- 1 to distance.toInt do
              current = (current._1, current._2 + 1)
              curLength += 1
              if !pathLength.contains(current) then
                pathLength(current) = curLength
          case "D" =>
            for i <- 1 to distance.toInt do
              current = (current._1, current._2 - 1)
              curLength += 1
              if !pathLength.contains(current) then
                pathLength(current) = curLength
          case "L" =>
            for i <- 1 to distance.toInt do
              current = (current._1 - 1, current._2)
              curLength += 1
              if !pathLength.contains(current) then
                pathLength(current) = curLength
          case "R" =>
            for i <- 1 to distance.toInt do
              current = (current._1 + 1, current._2)
              curLength += 1
              if !pathLength.contains(current) then
                pathLength(current) = curLength
  )
  pathLength

def everyCoordinate(instructions: List[String]): Set[(Int, Int)] = 
  val coordinates = Set[(Int, Int)]()
  var current = (0, 0)
  instructions.foreach(instruction =>
    val r = raw"(U|D|L|R)(\d+)".r
    instruction match
      case r(direction, distance) =>
        // println(s"$direction $distance")
        direction match
          case "U" =>
            for i <- 1 to distance.toInt do
              coordinates += ((current._1, current._2 + i))
            current = (current._1, current._2 + distance.toInt)
          case "D" =>
            for i <- 1 to distance.toInt do
              coordinates += ((current._1, current._2 - i))
            current = (current._1, current._2 - distance.toInt)
          case "L" =>
            for i <- 1 to distance.toInt do
              coordinates += ((current._1 - i, current._2))
            current = (current._1 - distance.toInt, current._2)
          case "R" =>
            for i <- 1 to distance.toInt do
              coordinates += ((current._1 + i, current._2))
            current = (current._1 + distance.toInt, current._2)
  )
  coordinates
