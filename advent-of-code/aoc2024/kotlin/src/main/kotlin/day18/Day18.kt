package engineer.kovalev.day18

import java.io.File
import java.util.LinkedList
import java.util.Queue

fun main() {
    part1()
    part2()
}

fun findPath(memory: List<List<Boolean>>): Int? {
    val lastIndex = memory.lastIndex

    val dirs = listOf(
        0 to -1,
        1 to 0,
        0 to 1,
        -1 to 0,
    )

    val visited = mutableSetOf(0 to 0)
    val queue: Queue<Triple<Int, Int, Int>> = LinkedList()
    queue.add(Triple(0, 0, 0))

    while (queue.isNotEmpty()) {
        val (x, y, l) = queue.remove()
        if (x == lastIndex && y == lastIndex) {
            return l
        }
        for ((dx, dy) in dirs) {
            val nextX = x + dx
            val nextY = y + dy

            if (nextY !in memory.indices || nextX !in memory[0].indices) {
                continue
            }
            if (nextX to nextY in visited) {
                continue
            }
            if (!memory[nextY][nextX]) {
                continue
            }

            queue.add(Triple(nextX, nextY, l + 1))
            visited.add(nextX to nextY)
        }
    }
    return null
}

fun part1() {
    val input = File("../input/18.txt").readText()
    val lastIndex = 70
    val limit = 1024
    val memory = MutableList(lastIndex + 1) { MutableList(lastIndex + 1) { true } }

    var fallen = 0
    for (line in input.trim().lines()) {
        val (x, y) = line.split(',').map { it.toInt() }
        memory[y][x] = false
        fallen++
        if (fallen == limit) {
            break
        }
    }

    println(findPath(memory))
}

fun part2() {
    val input = File("../input/18.txt").readText()
    val lastIndex = 70

    val memory = MutableList(lastIndex + 1) { MutableList(lastIndex + 1) { true } }

    var fallen = 0
    for (line in input.trim().lines()) {
        val (x, y) = line.split(',').map { it.toInt() }
        memory[y][x] = false
        fallen++
        if (findPath(memory) == null) {
            println("$x,$y")
            break
        }
    }
}