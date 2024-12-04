package engineer.kovalev.day04

import java.io.File
import kotlin.math.max
import kotlin.math.min

fun main() {
    part1()
    part2()
}

fun part1() {
    val lines = File("../input/04.txt").readLines()
    var res = 0
    for (i in lines.indices) {
        for (j in lines[i].indices) {
            res += findWords(lines, i, j, 4).count { it == "XMAS" }
        }
    }
    println(res)
}

fun part2() {
    val lines = File("../input/04.txt").readLines()
    var res = 0
    for (i in 1..<lines.size - 1) {
        for (j in 1..<lines[i].length - 1) {
            if (lines[i][j] != 'A') {
                continue
            }

            val primaryDiagonal = (lines[i-1][j-1] == 'M' && lines[i+1][j+1] == 'S') || (lines[i-1][j-1] == 'S' && lines[i+1][j+1] == 'M')
            if (!primaryDiagonal) {
                continue
            }

            val secondaryDiagonal = (lines[i-1][j+1] == 'M' && lines[i+1][j-1] == 'S') || (lines[i-1][j+1] == 'S' && lines[i+1][j-1] == 'M')
            if (secondaryDiagonal) {
                res++
            }
        }
    }
    println(res)
}

fun findWords(lines: List<String>, row: Int, col: Int, len: Int): List<String> {
    val words = mutableListOf<String>()

    // UP
    var word = ""
    for (i in row downTo max(0, row - len + 1)) {
        word += lines[i][col]
    }
    words.add(word)

    // RIGHT
    words.add(lines[row].substring(col, min(lines[row].length, col + 4)))

    // DOWN
    word = ""
    for (i in row .. min(lines.size - 1, row + len - 1)) {
        word += lines[i][col]
    }
    words.add(word)

    // LEFT
    word = ""
    for (j in col downTo max(0, col - len + 1)) {
        word += lines[row][j]
    }
    words.add(word)

    // UP-RIGHT
    word = ""
    for (idx in 0..< len) {
        if ((row - idx) >= 0 && (col + idx) < lines[row - idx].length) {
            word += lines[row-idx][col+idx]
        } else {
            break
        }
    }
    words.add(word)

    // DOWN-RIGHT
    word = ""
    for (idx in 0..< len) {
        if ((row + idx) < lines.size && (col + idx) < lines[row + idx].length) {
            word += lines[row+idx][col+idx]
        } else {
            break
        }
    }
    words.add(word)

    // DOWN-LEFT
    word = ""
    for (idx in 0..< len) {
        if ((row + idx) < lines.size && (col - idx) >= 0) {
            word += lines[row+idx][col-idx]
        } else {
            break
        }
    }
    words.add(word)

    // UP-LEFT
    word = ""
    for (idx in 0..< len) {
        if ((row - idx) >= 0 && (col - idx) >= 0) {
            word += lines[row-idx][col-idx]
        } else {
            break
        }
    }
    words.add(word)

    return words
}
