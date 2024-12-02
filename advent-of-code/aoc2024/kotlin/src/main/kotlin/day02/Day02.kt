package engineer.kovalev.day02

import java.io.File
import kotlin.math.abs

fun main() {
    val numbers = getNumbers()
    part1(numbers)
    part2(numbers)
}

fun getNumbers(): List<List<Int>> {
    val lines = File("../input/02.txt").readLines()
    val numbers = lines.map { line ->
        line.split("\\s+".toRegex()).map(String::toInt)
    }
    return numbers
}

fun part1(numbers: List<List<Int>>) {
    val safe = numbers.count { safe(it) }
    println(safe)
}

fun part2(reports: List<List<Int>>) {
    var safe = 0
    for (report in reports) {
        for (fixedReport in combinations(report)) {
            if (safe(fixedReport)) {
                safe++
                break
            }
        }
    }
    println(safe)
}

fun strictlyIncreasing(numbers: List<Int>): Boolean {
    return numbers.asSequence().windowed(2).all { (a, b) -> a < b }
}

fun strictlyDecreasing(numbers: List<Int>): Boolean {
    return numbers.asSequence().windowed(2).all { (a, b) -> a > b }
}

fun maxDiff(numbers: List<Int>): Int {
    return numbers.asSequence().windowed(2).map { (a, b) -> abs(a - b) }.max()
}

fun safe(numbers: List<Int>): Boolean {
    return (strictlyIncreasing(numbers) || strictlyDecreasing(numbers)) && maxDiff(numbers) <= 3
}

fun combinations(numbers: List<Int>): List<List<Int>> {
    val result = mutableListOf<List<Int>>()
    for (i in numbers.indices) {
        val l = numbers.toMutableList()
        l.removeAt(i)
        result.add(l)
    }
    return result
}
