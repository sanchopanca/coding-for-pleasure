package engineer.kovalev.day03

import java.io.File

fun main() {
    part1()
    part2()
    part2b()
}


fun part1() {
    val lines = File("../input/03.txt").readLines()
    val regex = "mul\\((\\d{1,3}),(\\d{1,3})\\)".toRegex()
    var result = 0
    for (line in lines) {
        val matches = regex.findAll(line);
        for (match in matches) {
            result += match.groupValues[1].toInt() * match.groupValues[2].toInt()
        }
    }
    println(result)
}

fun part2b() {  // doesn't work and i don't understand why
    var line = File("../input/03.txt").readText()
    line.replace("\n", "")
    val regex = "mul\\((\\d{1,3}),(\\d{1,3})\\)".toRegex()
    val remover = "don't\\(\\)(?!do\\(\\)).*?(do\\(\\)|\$)".toRegex()
    var result = 0
        while (remover.containsMatchIn(line)) {
            line = line.replace(remover, "")
            println("!!!$line")
        }
        val matches = regex.findAll(line);
        for (match in matches) {
            result += match.groupValues[1].toInt() * match.groupValues[2].toInt()
        }
    println(result)
}

fun part2() {
    val lines = File("../input/03.txt").readLines()
    val regex = "(mul\\((\\d{1,3}),(\\d{1,3})\\))|(do\\(\\))|(don't\\(\\))".toRegex()

    var result = 0
    var enabled = true;
    for (line in lines) {


        val matches = regex.findAll(line)
        for (match in matches) {
            if (enabled && match.groupValues[1] != "") {
                result += match.groupValues[2].toInt() * match.groupValues[3].toInt()
            }

            if (match.groupValues[4] != "") {
                enabled = true
            }

            if (match.groupValues[5] != "") {
                enabled = false
            }

        }
    }
    println(result)
}

// 98833043 wrong
// 71007585 wrong
// 110659571 wrong
// 94810037 wrong