package engineer.kovalev.day10

import java.io.File

fun main() {
    part1()
    part2()
}

fun part1() {
    val map = File("../input/10.txt").readLines()
    val zeros = findZeros(map)
    println(zeros.sumOf { findReachableNines(map, it.first, it.second).size })
}

fun part2() {
    val map = File("../input/10.txt").readLines()
    val zeros = findZeros(map)
    println(zeros.sumOf { findNumberOfPath(map, it.first, it.second) })
}

fun findZeros(map: List<String>): List<Pair<Int, Int>> {
    val result = mutableListOf<Pair<Int, Int>>()

    for (i in map.indices) {
        for (j in map[i].indices) {
            if (map[i][j] == '0') {
                result.add(Pair(i, j))
            }
        }
    }

    return result
}

fun findReachableNines(map: List<String>, x: Int, y: Int): Set<Pair<Int, Int>> {
    if (map[x][y] == '9') {
        return setOf(Pair(x, y))
    }

    val dirs = listOf(
        Pair(-1, 0),
        Pair(0, 1),
        Pair(1, 0),
        Pair(0, -1)
    )

    val result = mutableSetOf<Pair<Int, Int>>()
    for ((dx, dy) in dirs) {
        val (nextX, nextY) = x + dx to y + dy
        if (nextX !in map.indices || nextY !in map[0].indices) {
            continue
        }
        if (map[nextX][nextY].code - map[x][y].code == 1) {
            result.addAll(findReachableNines(map, nextX, nextY))
        }
    }
    return result
}

fun findNumberOfPath(map: List<String>, x: Int, y: Int): Int {
    if (map[x][y] == '9') {
        return 1
    }

    val dirs = listOf(
        Pair(-1, 0),
        Pair(0, 1),
        Pair(1, 0),
        Pair(0, -1)
    )

    var result = 0
    for ((dx, dy) in dirs) {
        val (nextX, nextY) = x + dx to y + dy
        if (nextX !in map.indices || nextY !in map[0].indices) {
            continue
        }
        if (map[nextX][nextY].code - map[x][y].code == 1) {
            result += findNumberOfPath(map, nextX, nextY)
        }
    }
    return result
}