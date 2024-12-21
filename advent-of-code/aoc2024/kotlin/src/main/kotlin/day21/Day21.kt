package engineer.kovalev.day21

import java.io.File

// valid only for my input,
// some combination are probably not optimal,
// but they are not in my input
val NUMPAD = mapOf(
    ('A' to 'A') to "A",
    ('A' to '0') to "<A",
    ('A' to '1') to "^<<A",
//    ('A' to '1') to "<^<A", wrong
    ('A' to '2') to "^<A",
    ('A' to '3') to "^A",
    ('A' to '4') to "^^<<A",
//    ('A' to '5') to "^^<A",  // don't have it, ramuuns has somehting else
    ('A' to '6') to "^^A",
    ('A' to '7') to "^^^<<A",
//    ('A' to '8') to "^^^<A", wrong
    ('A' to '8') to "<^^^A",
    ('A' to '9') to "^^^A",

    ('0' to 'A') to ">A",
    ('0' to '0') to "A",
    ('0' to '1') to "^<A",
    ('0' to '2') to "^A",
//    ('0' to '3') to "^>A",  // don't have it, ramuuns has something else
    ('0' to '4') to "^^<A",
    ('0' to '5') to "^^A",
//    ('0' to '6') to "^^>A",  // don't have it, ramuuns has something else
    ('0' to '7') to "^^^<A",
    ('0' to '8') to "^^^A",
//    ('0' to '9') to "^^^>A",  // don't have it, ramuuns has something else

    ('1' to 'A') to ">>vA",
    ('1' to '0') to ">vA",
    ('1' to '1') to "A",
    ('1' to '2') to ">A",
    ('1' to '3') to ">>A",
    ('1' to '4') to "^A",
//    ('1' to '5') to "^>A",  // don't have it, ramuuns has something else
    ('1' to '6') to "^>>A",
    ('1' to '7') to "^^A",
    ('1' to '8') to "^^>A",
    ('1' to '9') to "^^>>A",

//    ('2' to 'A') to "v>A",  // don't have it, ramuuns has something else
    ('2' to '0') to "vA",
    ('2' to '1') to "<A",
    ('2' to '2') to "A",
    ('2' to '3') to ">A",
//    ('2' to '4') to "<^A",  // don't have it, ramuuns has something else
    ('2' to '5') to "^A",
//    ('2' to '6') to "^>A",
//    ('2' to '7') to "<^^A",
    ('2' to '8') to "^^A",
    ('2' to '9') to "^^>A",  // ramuuns has something else
//    ('2' to '9') to ">^^A", // wrong?


    ('3' to 'A') to "vA",
    ('3' to '0') to "<vA",
    ('3' to '1') to "<<A",
    ('3' to '2') to "<A",
    ('3' to '3') to "A",
    ('3' to '4') to "<<^A",
    ('3' to '5') to "<^A",
    ('3' to '6') to "^A",
    ('3' to '7') to "<<^^A",
    ('3' to '8') to "<^^A",
//    ('3' to '8') to "^^<A", //wrong?
    ('3' to '9') to "^^A",

    ('4' to 'A') to ">>vvA",
    ('4' to '0') to ">vvA",
    ('4' to '1') to "vA",
//    ('4' to '2') to "v>A", // dont' have it, ramuuns has something else
//    ('4' to '3') to "v>>A", // dont' have it, ramuuns has something else
    ('4' to '4') to "A",
    ('4' to '5') to ">A",
    ('4' to '6') to ">>A",
    ('4' to '7') to "^A",
//    ('4' to '8') to "^>A", // dont' have it, ramuuns has something else
//    ('4' to '9') to "^>>A", // dont' have it, ramuuns has something else

    ('5' to 'A') to "vv>A",
    ('5' to '0') to "vvA",
    ('5' to '1') to "<vA",
    ('5' to '2') to "vA",
//    ('5' to '3') to "v>A", // dont' have it, ramuuns has something else
    ('5' to '4') to "<A",
    ('5' to '5') to "A",
    ('5' to '6') to ">A",
    ('5' to '7') to "<^A",
    ('5' to '8') to "^A",
    ('5' to '9') to "^>A",

    ('6' to 'A') to "vvA",
    ('6' to '0') to "<vvA",
    ('6' to '1') to "<<vA",
    ('6' to '2') to "<vA",
    ('6' to '3') to "vA",
    ('6' to '4') to "<<A",
    ('6' to '5') to "<A",
    ('6' to '6') to "A",
    ('6' to '7') to "<<^A",
    ('6' to '8') to "<^A",
    ('6' to '9') to "^A",

    ('7' to 'A') to ">>vvvA",
    ('7' to '0') to ">vvvA",
    ('7' to '1') to "vvA",
    ('7' to '2') to "vv>A",
    ('7' to '3') to "vv>>A",
    ('7' to '4') to "vA",
    ('7' to '5') to "v>A",
    ('7' to '6') to "v>>A",
    ('7' to '7') to "A",
    ('7' to '8') to ">A",
    ('7' to '9') to ">>A",

    ('8' to 'A') to "vvv>A",
    ('8' to '0') to "vvvA",
    ('8' to '1') to "<vvA",
    ('8' to '2') to "vvA",
    ('8' to '3') to "vv>A",
    ('8' to '4') to "<vA",
    ('8' to '5') to "vA",
    ('8' to '6') to "v>A",
    ('8' to '7') to "<A",
    ('8' to '8') to "A",
    ('8' to '9') to ">A",

    ('9' to 'A') to "vvvA",
    ('9' to '0') to "<vvvA",
    ('9' to '1') to "<<vvA",
    ('9' to '2') to "<vvA",
    ('9' to '3') to "vvA",
    ('9' to '4') to "<<vA",
    ('9' to '5') to "<vA",
    ('9' to '6') to "vA",
    ('9' to '7') to "<<A",
    ('9' to '8') to "<A",
    ('9' to '9') to "A",
)

val DIRECTIONAL_PAD = mapOf(
    ('A' to 'A') to "A",
    ('A' to '<') to "v<<A",
//    ('A' to '<') to "<v<A",
    ('A' to 'v') to "<vA",
//    ('A' to 'v') to "v<A",
    ('A' to '>') to "vA",
    ('A' to '^') to "<A",

    ('<' to 'A') to ">>^A",
//    ('<' to 'A') to ">^>A",
    ('<' to '<') to "A",
    ('<' to 'v') to ">A",
    ('<' to '>') to ">>A",
    ('<' to '^') to ">^A",

//    ('v' to 'A') to ">^A", // too high
    ('v' to 'A') to "^>A",  // too low
    ('v' to '<') to "<A",
    ('v' to 'v') to "A",
    ('v' to '>') to ">A",
    ('v' to '^') to "^A",

    ('>' to 'A') to "^A",
    ('>' to '<') to "<<A",
    ('>' to 'v') to "<A",
    ('>' to '>') to "A",
    ('>' to '^') to "<^A",
//    ('>' to '^') to "^<A", // wrong

    ('^' to 'A') to ">A",
    ('^' to '<') to "v<A",
    ('^' to 'v') to "vA",
    ('^' to '>') to "v>A",
//    ('^' to '>') to ">vA", // wrong
    ('^' to '^') to "A",
)

val NUMPAD_ = """
    789
    456
    123
     0A
""".trimIndent().lines()

val DPAD = """
    | ^A
    |<v>
""".trimMargin().lines()


fun mapPresses(input: String, map: Map<Pair<Char, Char>, String>): String {
    var result = ""
    var current = 'A'
    for (c in input) {
        result += map[current to c]
        current = c
    }
    return result
}

//fun mapPresses2(input: String, pad: List<String>): List<String> {
//    var results = setOf<String>()
//    var current = 'A'
//    for (c in input) {
//
//    }
//    return result
//}

fun n(input: String): String {
    return mapPresses(input, NUMPAD)
}

fun d(input: String): String {
    return mapPresses(input, DIRECTIONAL_PAD)
}

val cache = mutableMapOf<Pair<String, Int>, Long>()
fun d(input: String, n: Int): Long {
    if (input.last() != 'A') {
        throw Exception("assertion failed")  // assert() does nothing by default, even in debug. wtf?
    }
    if (input to n in cache) {
        return cache[input to n]!!
    }

    if (n == 1) {
        val result = d(input).length.toLong()
        cache[input to n] = result
        return result
    }

    val parts = input.split('A').filter { it != "" }.map { it + "A" }
    var result = 0L
    for (part in parts) {
        val expanded = d(part)
        result += d(expanded, n - 1)
    }
    cache[input to n] = result
    return result
}

fun main() {
    println(DPAD)
    val content = File("../input/21.txt").readText()
//    val content = """029A
//980A
//179A
//456A
//379A"""
//    println("zxvA123A".split('A'))
//    val c = "179A"
//    println(n(c).length)
//    println(d(n(c)).length)
//    println(applyNTimes(::d, n(c), 2).length)
//    println(applyNTimes(::d, n(c), 3).length)
//    println(applyNTimes(::d, n(c), 4).length)
//    println(applyNTimes(::d, n(c), 5).length)
//    println(applyNTimes(::d, n(c), 6).length)
//    println(applyNTimes(::d, n(c), 7).length)
//    println(applyNTimes(::d, n(c), 8).length)
//    println(applyNTimes(::d, n(c), 9).length)
    var result = 0
    var result2 = 0L
    for (code in content.trim().lines()) {
        val seq = d(d(n(code)))
        val l = d(n(code), 25)
        val n = code.slice(0..2).toInt()
        result += n * seq.length
        result2 += n * l
    }
    println(result)
    println(result2)

    println(findPaths(NUMPAD_, '1', '5')) // Expected: listOf(">^", "^>")
    println(findPaths(NUMPAD_, '0', '1')) // Expected: listOf("^<")
}

fun applyNTimes(fn: (String) -> String, input: String, n: Int): String {
    var result = input
    repeat(n) { result = fn(result) }
    return result
}

// 292475867313629 too high
// 256357883649937 too low

fun findPaths(grid: List<String>, start: Char, end: Char): List<String> {
    val directions = listOf(
        Pair(-1, 0) to "^", // Up
        Pair(1, 0) to "v",  // Down
        Pair(0, -1) to "<", // Left
        Pair(0, 1) to ">"   // Right
    )

    data class State(val x: Int, val y: Int, val path: String, val visited: Set<Pair<Int, Int>>)

    fun findChar(c: Char): Pair<Int, Int>? {
        for (i in grid.indices) {
            for (j in grid[i].indices) {
                if (grid[i][j] == c) return Pair(i, j)
            }
        }
        return null
    }

    val startPos = findChar(start) ?: return emptyList()
    val endPos = findChar(end) ?: return emptyList()

    val queue = ArrayDeque<State>()
    val shortestPaths = mutableListOf<String>()
    var shortestLength = Int.MAX_VALUE

    queue.add(State(startPos.first, startPos.second, "", setOf(startPos)))

    while (queue.isNotEmpty()) {
        val (x, y, path, visited) = queue.removeFirst()

        // Stop exploring if we've exceeded the shortest path length found
        if (path.length > shortestLength) continue

        if (Pair(x, y) == endPos) {
            if (path.length < shortestLength) {
                shortestPaths.clear()
                shortestLength = path.length
            }
            shortestPaths.add(path)
            continue
        }

        for ((delta, direction) in directions) {
            val newX = x + delta.first
            val newY = y + delta.second

            if (newX in grid.indices && newY in grid[newX].indices &&
                grid[newX][newY] != ' ' &&
                Pair(newX, newY) !in visited
            ) {
                queue.add(State(newX, newY, path + direction, visited + Pair(newX, newY)))
            }
        }
    }

    return shortestPaths
}