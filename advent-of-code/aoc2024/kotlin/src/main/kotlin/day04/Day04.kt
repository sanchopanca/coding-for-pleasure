package engineer.kovalev.day04

import java.io.File

fun main() {
    part1()
    part2()
}

fun part1() {
    val lines = File("../input/04.txt").readLines()
    val targetWord = "XMAS"
    val result = lines.indices.sumOf { i ->
        lines[i].indices.sumOf { j ->
            findWords(lines, i, j, targetWord.length).count { it == targetWord }
        }
    }
    println(result)
}

fun part2() {
    val lines = File("../input/04.txt").readLines()
    val result = (1 ..< lines.size - 1).sumOf { i ->
        (1 ..< lines[i].length - 1).count { j ->
            lines[i][j] == 'A' &&
                    isDiagonalMatch(lines, i, j)
        }
    }
    println(result)
}

fun isDiagonalMatch(lines: List<String>, row: Int, col: Int): Boolean {
    val primary = (lines[row - 1][col - 1] == 'M' && lines[row + 1][col + 1] == 'S') ||
            (lines[row - 1][col - 1] == 'S' && lines[row + 1][col + 1] == 'M')
    val secondary = (lines[row - 1][col + 1] == 'M' && lines[row + 1][col - 1] == 'S') ||
            (lines[row - 1][col + 1] == 'S' && lines[row + 1][col - 1] == 'M')
    return primary && secondary
}

fun findWords(lines: List<String>, row: Int, col: Int, len: Int): List<String> {
    val directions = listOf(
        Pair(-1, 0), // UP
        Pair(0, 1),  // RIGHT
        Pair(1, 0),  // DOWN
        Pair(0, -1), // LEFT
        Pair(-1, 1), // UP-RIGHT
        Pair(1, 1),  // DOWN-RIGHT
        Pair(1, -1), // DOWN-LEFT
        Pair(-1, -1) // UP-LEFT
    )

    return directions.map { (dr, dc) ->
        StringBuilder().apply {
            for (step in 0 ..< len) {
                val newRow = row + step * dr
                val newCol = col + step * dc
                if (newRow in lines.indices && newCol in lines[newRow].indices) {
                    append(lines[newRow][newCol])
                } else {
                    break
                }
            }
        }.toString()
    }
}
