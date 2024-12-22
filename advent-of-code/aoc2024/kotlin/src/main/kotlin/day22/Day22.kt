package engineer.kovalev.day22

import java.io.File
import kotlin.math.max

data class Diffs(val a: Int, val b: Int, val c: Int, val d: Int)

val cache = mutableMapOf<Long, Map<Diffs, Int>>()

fun part1() {
    val content = File("../input/22.txt").readText()
    var result = 0L
    for (line in content.trim().lines()) {
        var n = line.toLong()
        repeat(20000) { n = next(n) }
        result += n
    }
    println(result)
}

fun part2() {
    val content = File("../input/22.txt").readText()
    var maxResult = 0L
    val numbers = content.trim().lines().map { it.toLong() }.toList()
    for (n in numbers) {
        fillCache(n)
    }
    for (a in -9..9) {
        for (b in -9..9) {
            for (c in -9..9) {
                for (d in -9..9) {
                    if (!possible(a, b, c, d)) {
                        continue
                    }
                    var result = 0L
                    val diffs = Diffs(a, b, c, d)
                    for (n in numbers) {
                        result += cache[n]!!.getOrDefault(diffs, 0).toLong()
                    }
                    maxResult = max(maxResult, result)
                }
            }
        }
    }
    println(maxResult)
}

fun next(n: Long): Long {
    var r = (n xor (n * 64)) % 16777216
    r = (r xor (r / 32)) % 16777216
    r = (r xor (r * 2048)) % 16777216
    return r
}

fun fillCache(n: Long) {
    val map = mutableMapOf<Diffs, Int>()
    val queue = FixedSizeQueue(4)
    var curN = n
    for(x in 1..2000) {
        val nextN = next(curN)
        queue.add((nextN % 10 - curN % 10).toInt())
        if (queue.size < 4) {
            curN = nextN
            continue
        }

        val diffs = queue.toDiffs()
        if (diffs in map) {
            curN = nextN
            continue
        }

        map[diffs] = (nextN % 10).toInt()
        curN = nextN
    }
    cache[n] = map
}

fun possible(a: Int, b: Int, c: Int, d: Int): Boolean {
    var n = a
    n += b
    if (n > 9) {
        return false
    }
    n += c
    if (n > 9) {
        return false
    }
    n += d
    if (n > 9) {
        return false
    }

    n = 9 + a

    n += b
    if (n < 0) {
        return false
    }

    n += c
    if (n < 0) {
        return false
    }

    n += d
    return n >= 0
}

fun main() {
    part1()
    part2()
}

class FixedSizeQueue(private val maxSize: Int) {
    private val deque: ArrayDeque<Int> = ArrayDeque()

    val size: Int
        get() = deque.size

    fun add(element: Int) {
        if (deque.size == maxSize) {
            deque.removeFirst()
        }
        deque.addLast(element)
    }

    private fun toList(): List<Int> = deque.toList()

    fun toDiffs(): Diffs {
        if (this.size != 4) {
            throw Exception("can't")
        }

        val (a, b, c, d) = this.toList()
        return Diffs(a, b, c, d)
    }
}