package engineer.kovalev.day25

import java.io.File

fun main() {
    val content = File("../input/25.txt").readText()

    val schemas = content.trim().split("\n\n")

    val locks = mutableListOf<List<Int>>()
    val keys = mutableListOf<List<Int>>()
    for (s in schemas) {
        val schema = s.lines()
        if (schema[0][0] == '#') {
            locks.add(parseLock(schema))
        } else {
            keys.add(parseKey(schema))
        }
    }

    var fit = 0
    for (lock in locks) {
        loop@for (key in keys) {
            for (i in key.indices) {
                if (lock[i] + key[i] > 5) {
                    continue@loop
                }
            }
            fit++
        }
    }
    println(fit)
}

fun parseLock(schema: List<String>): List<Int> {
    val result = mutableListOf<Int>()
    for (j in schema[0].indices) {
        var n = -1
        for (i in schema.indices) {
            if (schema[i][j] == '#') {
                n++
            }
            else {
                result.add(n)
                break
            }
        }
    }
    return result
}

fun parseKey(schema: List<String>): List<Int> {
    val result = mutableListOf<Int>()
    for (j in schema[0].indices) {
        var n = -1
        for (i in schema.indices.reversed()) {
            if (schema[i][j] == '#') {
                n++
            }
            else {
                result.add(n)
                break
            }
        }
    }
    return result
}