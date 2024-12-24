package engineer.kovalev.day24

import engineer.kovalev.day05.swap
import java.io.File

fun main() {
    val (calculated, expressions) = parseInput("../input/24.txt")
    println(getZ(calculated, expressions))
    makeDotFile()
    part2(expressions)
}

fun part2(expressions: MutableMap<String, String>) {
    expressions.swap("z05", "hdt")
    expressions.swap("z09", "gbf")
    expressions.swap("mht", "jgt")
    expressions.swap("z30", "nbf")
    val x = "0100010000 1011101101 1011101101 0100010000".replace(" ", "").toLong(2)
    val y = "1011101111 0100010010 0100010010 1011101111".replace(" ", "").toLong(2)
    val values = setValues(x, y)
    println(getZ(values, expressions).toString(2))
    println(listOf("z05", "hdt", "z09", "gbf", "mht", "jgt", "z30", "nbf").sorted().joinToString(","))
}

fun setValues(x: Long, y: Long): MutableMap<String, Boolean> {
    val values = zero()
    for (bit in 0..44) {
        val xHasBitSet = ((x shr bit) and 1) == 1L
        val yHasBitSet = ((y shr bit) and 1) == 1L
        values["x${bit.pad()}"] = xHasBitSet
        values["y${bit.pad()}"] = yHasBitSet
    }
    return values
}

fun parseInput(filePath: String): Pair<MutableMap<String, Boolean>, MutableMap<String, String>> {
    val content = File(filePath).readText()
    val (p1, p2) = content.split("\n\n")

    val calculated = mutableMapOf<String, Boolean>()
    for (line in p1.lines()) {
        val (name, value) = line.split(": ")
        calculated[name] = value != "0"
    }

    val expressions = mutableMapOf<String, String>()
    for (line in p2.trim().lines()) {
        val (expr, name) = line.split(" -> ")
        expressions[name] = expr
    }

    return Pair(calculated, expressions)
}

fun makeDotFile() {

    val content = File("../input/24.txt").readText()
    val (_, input) = content.split("\n\n")
    val output = StringBuilder()
    output.append("digraph G {\n")

    val operationColors = mapOf(
        "AND" to "red",
        "OR" to "blue",
        "XOR" to "green"
    )

    input.trim().lines().forEach { line ->
        val parts = line.split(" ")
        val input1 = parts[0]
        val operation = parts[1]
        val input2 = parts[2]
        val outputNode = parts[4]

        val color = operationColors[operation] ?: "black"
        output.append("    $input1 -> $outputNode [color=$color];\n")
        output.append("    $input2 -> $outputNode [color=$color];\n")
    }

    output.append("}")

    File("out.dot").writeText(output.toString())
}

fun getZ(calculated: MutableMap<String, Boolean>, expressions: Map<String, String>): Long {
    var binary = ""
    for (i in (0..45).reversed()) {
        val v = "z" + i.toString().padStart(2, '0')
        binary += if (getValue(v, calculated, expressions)) {
            "1"
        } else {
            "0"
        }
    }
    return binary.toLong(2)
}

fun swap(expressions: MutableMap<String, String>, from: String, to: String) {
    val tmp = expressions[from]!!
    expressions[from] = expressions[to]!!
    expressions[to] = tmp
}

fun zero(): MutableMap<String, Boolean> {
    val map = mutableMapOf<String, Boolean>()
    for (i in 0..44) {
        map["x${i.pad()}"] = false
        map["y${i.pad()}"] = false
    }
    return map
}


fun getValue(v: String, calculated: MutableMap<String, Boolean>, expressions: Map<String, String>): Boolean {
    if (v in calculated) {
        return calculated[v]!!
    }

    val expr = expressions[v]!!
    val (a, op, b) = expr.split(' ')

    val v1 = getValue(a, calculated, expressions)
    val v2 = getValue(b, calculated, expressions)

    return when (op) {
        "AND" -> v1 && v2
        "OR" -> v1 || v2
        "XOR" -> v1 != v2
        else -> {
            throw Exception("unreachable")
        }
    }
}

fun Boolean.toInt() = if (this) 1 else 0

fun Int.pad() = this.toString().padStart(2, '0')