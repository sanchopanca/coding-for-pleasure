package engineer.kovalev.day20

import engineer.kovalev.day06.replaceCharAt
import java.io.File
import java.util.*
import kotlin.math.abs

fun main() {
    val content = File("../input/20.txt").readText()
    val map = content.trim().lines()
    val (s, e) = findStartAndEnd(map)
    val withoutCheats = findPathLength(map, s, e)
    var part1 = 0
    for (y in map.indices) {
        for (x in map[0]. indices) {
            val withCheats = findPathLength(removeWall(map, y, x), s, e)
            if (withoutCheats - withCheats >= 100) {
                part1++
            }
        }
    }
    println(part1)

    var part2 = 0
    for (y in map.indices) {
        println(y)
        for (x in map[0].indices) {
            for (y2 in map.indices) {
                for (x2 in map[0].indices) {
                    val p1 = x to y
                    val p2 = x2 to y2
                    if (canCheat(map, p1, p2)) {
                        val withCheats = findPathLength(map, s, e, p1, p2)
                        if (withoutCheats - withCheats >= 100) {
                            part2++
                        }
                    }
                }
            }
        }
    }
    println(part2)
}

fun findStartAndEnd(map: List<String>): Pair<Pair<Int, Int>, Pair<Int, Int>> {
    var (sX, sY) = 0 to 0
    var (eX, eY) = 0 to 0
    for ((y, line) in map.withIndex()) {
        for ((x, c) in line.withIndex()) {
            if (c == 'S') {
                sX = x
                sY = y
            }
            if (c == 'E') {
                eX = x
                eY = y
            }
        }
    }
    return Pair(sX to sY, eX to eY)
}

fun findPathLength(map: List<String>, from: Pair<Int, Int>, to: Pair<Int, Int>): Int {
    val (eX, eY) = to

    val dirs = listOf(
        0 to -1,
        1 to 0,
        0 to 1,
        -1 to 0,
    )

    val visited = mutableSetOf(from)
    val queue: Queue<Triple<Int, Int, Int>> = LinkedList()
    queue.add(Triple(from.first, from.second, 0))

    while (queue.isNotEmpty()) {
        val (x, y, l) = queue.remove()
        if (x == eX && y == eY) {
            return l
        }
        for ((dx, dy) in dirs) {
            val nextX = x + dx
            val nextY = y + dy

            if (nextY !in map.indices || nextX !in map[0].indices) {
                continue
            }
            if (nextX to nextY in visited) {
                continue
            }
            if (map[nextY][nextX] == '#') {
                continue
            }

            queue.add(Triple(nextX, nextY, l + 1))
            visited.add(nextX to nextY)
        }
    }
    throw Exception("unreachable")
}

data class Step(val x: Int, val y: Int, val lengthFromStart: Int, val cheatEnabled: Boolean, val cheatRemaining: Int, val enabledAt: Int, val disabledAt: Int)

fun findPathLength(map: List<String>, from: Pair<Int, Int>, to: Pair<Int, Int>, cheatFrom: Pair<Int, Int>, cheatTo: Pair<Int, Int>): Int {
    val (eX, eY) = to

    val dirs = listOf(
        0 to -1,
        1 to 0,
        0 to 1,
        -1 to 0,
    )

    val visited = mutableSetOf(from)
    val queue: Queue<Triple<Int, Int, Int>> = LinkedList()
    queue.add(Triple(from.first, from.second, 0))

    while (queue.isNotEmpty()) {
        val (x, y, l) = queue.remove()
        if (x == eX && y == eY) {
            return l
        }
        if (x to y == cheatFrom) {
            queue.add(Triple(cheatTo.first, cheatTo.second, l+cheatTo.distanceTo(cheatFrom)))
            visited.add(cheatTo)
        }
        for ((dx, dy) in dirs) {
            val nextX = x + dx
            val nextY = y + dy

            if (nextY !in map.indices || nextX !in map[0].indices) {
                continue
            }
            if (nextX to nextY in visited) {
                continue
            }
            if (map[nextY][nextX] == '#') {
                continue
            }

            queue.add(Triple(nextX, nextY, l + 1))
            visited.add(nextX to nextY)
        }
    }
    throw Exception("unreachable")
}

fun removeWall(map: List<String>, y: Int, x: Int): List<String> {
    val newMap = map.toMutableList()
    val newRow = map[y].replaceCharAt(x, '.')
    newMap[y] = newRow
    return newMap
}

fun Pair<Int, Int>.distanceTo(other: Pair<Int, Int>): Int {
    return abs(this.first - other.first) + abs(this.second - other.second)
}

fun canCheat(map: List<String>, from: Pair<Int, Int>, to: Pair<Int, Int>): Boolean {
    val l = from.distanceTo(to)
    if (l > 20) {
        return false
    }

    if (map[from.second][from.first] == '#' || map[to.second][to.first] == '#') {
        return false
    }
    return true
}
