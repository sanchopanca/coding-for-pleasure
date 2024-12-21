package engineer.kovalev.day21

import java.io.File

data class Vector(val x: Int, val y: Int)

object Constants {
    val directions = mapOf(
        "^" to Vector(0, -1),
        ">" to Vector(1, 0),
        "v" to Vector(0, 1),
        "<" to Vector(-1, 0)
    )

    val numericKeypad = mapOf(
        "9" to Vector(2, 0),
        "8" to Vector(1, 0),
        "7" to Vector(0, 0),
        "6" to Vector(2, 1),
        "5" to Vector(1, 1),
        "4" to Vector(0, 1),
        "3" to Vector(2, 2),
        "2" to Vector(1, 2),
        "1" to Vector(0, 2),
        "0" to Vector(1, 3),
        "X" to Vector(0, 3),
        "A" to Vector(2, 3)
    )

    val directionalKeypad = mapOf(
        "X" to Vector(0, 0),
        "^" to Vector(1, 0),
        "A" to Vector(2, 0),
        "<" to Vector(0, 1),
        "v" to Vector(1, 1),
        ">" to Vector(2, 1)
    )
}

data class Position(val x: Int, val y: Int, val path: String)

fun findShortestPaths(input: Map<String, Vector>, start: String, end: String): List<String> {
    if (start == end) return listOf("A")

    val queue = mutableListOf(Position(input[start]!!.x, input[start]!!.y, ""))
    val distances = mutableMapOf<String, Int>()
    val allPaths = mutableListOf<String>()

    while (queue.isNotEmpty()) {
        val current = queue.removeFirst()
        val (x, y, path) = current

        if (x == input[end]!!.x && y == input[end]!!.y) {
            allPaths.add("${path}A")
            continue
        }

        val positionKey = "$x,$y"
        if (positionKey in distances && distances[positionKey]!! < path.length) {
            continue
        }

        for ((direction, vector) in Constants.directions) {
            val newX = x + vector.x
            val newY = y + vector.y

            if (input["X"]!!.x == newX && input["X"]!!.y == newY) {
                continue
            }

            for ((button, pos) in input) {
                if (pos.x == newX && pos.y == newY) {
                    val newPath = "$path$direction"
                    val newKey = "$newX,$newY"
                    if (newKey !in distances || distances[newKey]!! >= newPath.length) {
                        distances[newKey] = newPath.length
                        queue.add(Position(newX, newY, newPath))
                    }
                }
            }
        }
    }

    return allPaths.sortedBy { it.length }
}

fun getKeyPresses(
    input: Map<String, Vector>,
    code: String,
    robot: Int,
    memo: MutableMap<String, Long>
): Long {
    val key = "$code,$robot"
    memo[key]?.let { return it }

    var current = "A"
    var length = 0L

    for (char in code) {
        val moves = findShortestPaths(input, current, char.toString())
        if (robot == 0) {
            length += moves[0].length
        } else {
            var minLength = Long.MAX_VALUE
            for (move in moves) {
                minLength = minOf(
                    minLength,
                    getKeyPresses(Constants.directionalKeypad, move, robot - 1, memo)
                )
            }
            length += minLength
        }
        current = char.toString()
    }

    memo[key] = length
    return length
}

fun calculateTotalComplexity(input: String): Long {
    val keycodes = input.trim().split("\n")
    val memo = mutableMapOf<String, Long>()
    var total = 0L

    for (code in keycodes) {
        val numerical = code.filter { it.isDigit() }.toLong()
        total += numerical * getKeyPresses(Constants.numericKeypad, code, 25, memo)
    }

    return total
}

fun calculateComplexity(filePath: String) {
    val input = File(filePath).readText()
    val totalComplexity = calculateTotalComplexity(input)
    println("Total complexity for ${File(filePath).name}: $totalComplexity")
}

fun main() {
    calculateComplexity("../input/21.txt")
}