package engineer.kovalev.day14

import java.io.File

fun main() {
    part1()
    part2()
}

fun part1() {
    val width = 101
    val height = 103
    val seconds = 100
    val regex = Regex("-?\\d+")
    val robots = File("../input/14.txt").readLines()
    val q = mutableListOf(mutableListOf(0, 0), mutableListOf(0, 0))
    for (line in robots) {
        var (x, y, vx, vy) = regex.findAll(line).map { it.value.toInt() }.toList()
        x = (x + seconds * vx).mod(width)
        y = (y + seconds * vy).mod(height)

        var robot = 1
        val i = if (x < width / 2) {
            0
        } else if (x > width / 2) {
            1
        } else {
            robot = 0
            0
        }

        val j = if (y < height / 2) {
            0
        } else if (y > height / 2) {
            1
        } else {
            robot = 0
            0
        }
        q[j][i] += robot

    }
    println(q[0][0] * q[0][1] * q[1][0] * q[1][1])
}

fun part2() {
    val width = 101
    val height = 103
    val regex = Regex("-?\\d+")
    val robots = File("../input/14.txt").readLines()
    for (seconds in 1..20000) {
        val floor = MutableList(height, { MutableList(width, {'.'}) })
        for (line in robots) {
            var (x, y, vx, vy) = regex.findAll(line).map { it.value.toInt() }.toList()
            x = (x + seconds * vx).mod(width)
            y = (y + seconds * vy).mod(height)

            floor[y][x] = '#'
        }

        for (line in floor) {
            if ("########" in line.joinToString("")) {
                println(seconds)
                return
            }
        }
    }
}