package engineer.kovalev.day06

import java.io.File

fun main() {
    val path = "../input/06.txt"
    val (visited, starting) = part1(path)
    part2(path, visited, starting)
}

fun part1(path: String): Pair<Set<Pair<Int, Int>>, Pair<Int, Int>> {
    val map = File(path).readLines()
    var (x, y) = intArrayOf(0, 0)

    loop@ for (i in map.indices) {
        for (j in map.indices) {
            if (map[i][j] == '^') {
                x = i
                y = j
                break@loop
            }
        }
    }

    val starting = Pair(x, y)

    val visited = mutableSetOf<Pair<Int, Int>>()

    val turn = sequence {
        while (true) {
            yield(listOf(-1, 0))
            yield(listOf(0, 1))
            yield(listOf(1, 0))
            yield(listOf(0, -1))
        }
    }.iterator()

    var (dx, dy) = turn.next()

    while (x in map.indices && y in map[0].indices) {
        visited.add(Pair(x, y))
        if (map.getOrElse(x+dx) { "" }.getOrElse(y+dy) { '.' } == '#') {
            val nextTurn = turn.next()  // kotlin wtf, why destructuring only works in declarations
            dx = nextTurn[0]
            dy = nextTurn[1]
        }
        x += dx
        y += dy
    }

    println(visited.size)

    visited.remove(starting)

    return Pair(visited, starting)
}

fun part2(path: String, obstacles: Set<Pair<Int, Int>>, starting: Pair<Int, Int>) {
    val originalMap = File(path).readLines()
    var result = 0
    loop@ for ((ox, oy) in obstacles) {
        val map = originalMap.toMutableList()
        map[ox] = map[ox].replaceCharAt(oy, '#')

        val visited = mutableSetOf<Pair<Pair<Int, Int>, Pair<Int, Int>>>()

        val turn = sequence {
            while (true) {
                yield(listOf(-1, 0))
                yield(listOf(0, 1))
                yield(listOf(1, 0))
                yield(listOf(0, -1))
            }
        }.iterator()

        var (dx, dy) = turn.next()

        var (x, y) = starting
        while (x in map.indices && y in map[0].indices) {
            val justVisited = Pair(Pair(x, y), Pair(dx, dy))
            if (justVisited in visited) {
                result++
                continue@loop
            }
            visited.add(justVisited)
            if (map.getOrElse(x+dx) { "" }.getOrElse(y+dy) { '.' } == '#') {
                val nextTurn = turn.next()  // kotlin wtf, why destructuring only works in declarations
                dx = nextTurn[0]
                dy = nextTurn[1]
                visited.remove(justVisited)
                continue
            }
            x += dx
            y += dy
        }
    }
    println(result)
}

fun String.replaceCharAt(index: Int, newChar: Char): String {
    if (index < 0 || index >= this.length) {
        throw IndexOutOfBoundsException("Index $index is out of bounds for string of length ${this.length}")
    }
    return this.substring(0, index) + newChar + this.substring(index + 1)
}
