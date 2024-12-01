package engineer.kovalev.day01

import java.io.File
import kotlin.math.abs

fun main() {
    val lines = File("../input/01.txt").readLines()
    part1(lines)
    part2(lines)
}

fun part1(lines: List<String>) {
    val numbers = lines.map { line ->
        line.split("\\s+".toRegex()).map(String::toInt)
    }

    val primaryNumbers = numbers.map { it[0] }.sorted()
    val secondaryNumbers = numbers.map { it[1] }.sorted()

    val difference = primaryNumbers.indices.sumOf { abs(primaryNumbers[it] - secondaryNumbers[it]) }

    println(difference)
}

fun part2(lines: List<String>) {
    val frequencyMap1 = mutableMapOf<Int, Int>()
    val frequencyMap2 = mutableMapOf<Int, Int>()

    lines.forEach { line ->
        val (n1, n2) = line.split("\\s+".toRegex()).map(String::toInt)
        frequencyMap1[n1] = frequencyMap1.getOrDefault(n1, 0) + 1
        frequencyMap2[n2] = frequencyMap2.getOrDefault(n2, 0) + 1
    }

    val similarityScore = frequencyMap1.entries.sumOf { (number, count) ->
        count * number * frequencyMap2.getOrDefault(number, 0)
    }

    println(similarityScore)
}