package engineer.kovalev.day11

import java.io.File


fun main() {
    val numbers = File("../input/11.txt").readText().trim().split(' ').map { it.toLong() }
    println(numbers.sumOf { solve(it, 25) })
    println(numbers.sumOf { solve(it, 75) })
}

val cache = mutableMapOf<Pair<Long, Int>, Long>()

fun solve(n: Long, steps: Int): Long {
    val args = Pair(n, steps)
    if (args in cache) {
        return cache[args]!!
    }

    val s = n.toString()
    val l = s.length
    val result = if (steps == 0) {
        1L
    } else if (n == 0L) {
        solve(1, steps - 1)
    } else if (l % 2 == 0) {
        solve(s.substring(0, l / 2).toLong(), steps - 1) + solve(s.substring(l / 2).toLong(), steps - 1)
    } else {
        solve(n * 2024, steps - 1)
    }

    cache[args] = result
    return result
}