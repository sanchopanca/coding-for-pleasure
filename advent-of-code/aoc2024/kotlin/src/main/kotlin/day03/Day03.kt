package engineer.kovalev.day03

import java.io.File

fun main() {
    part1()
    part2()
    part2b()
}


fun part1() {
    val lines = File("../input/03.txt").readLines()
    val regex = """mul\((\d{1,3}),(\d{1,3})\)""".toRegex()
    var result = 0
    for (line in lines) {
        val matches = regex.findAll(line);
        for (match in matches) {
            val (a, b) = match.destructured
            result += a.toInt() * b.toInt()
        }
    }
    println(result)
}

fun part2() {
    val regex = """(?<mul>mul\((\d{1,3}),(\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))""".toRegex()
    var isProcessingEnabled = true
    var totalSum = 0

    File("../input/03.txt").forEachLine { line ->
        regex.findAll(line).forEach { match ->
            when {
                match.groups["mul"] != null && isProcessingEnabled -> {
                    val (_, a, b) = match.destructured
                    totalSum += a.toInt() * b.toInt()
                }
                match.groups["do"] != null -> isProcessingEnabled = true
                match.groups["dont"] != null -> isProcessingEnabled = false
            }
        }
    }
    println(totalSum)
}

fun part2b() {  // doesn't work and i don't understand why
    var line = File("../input/03.txt").readText()
    line.replace("\n", "")
    val regex = "mul\\((\\d{1,3}),(\\d{1,3})\\)".toRegex()
    val remover = "don't\\(\\)(?!do\\(\\)).*?(do\\(\\)|\$)".toRegex()
    var result = 0
    while (remover.containsMatchIn(line)) {
        line = line.replace(remover, "")
    }
    val matches = regex.findAll(line);
    for (match in matches) {
        result += match.groupValues[1].toInt() * match.groupValues[2].toInt()
    }
    println("Wrong: $result")
}