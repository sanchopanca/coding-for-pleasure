package engineer.kovalev.day01

import java.io.File
import kotlin.math.abs

fun main() {
    part1()
    part2()
}

fun part1() {
    val lines = File("../input/01.txt").readLines()
    val list1 = mutableListOf<Int>()
    val list2 = mutableListOf<Int>()
    for (line in lines) {
        val numbers = line.split("\\s+".toRegex())
        list1.addLast(numbers[0].toInt())
        list2.addLast(numbers[1].toInt())
    }
    list1.sort()
    list2.sort()

    var diff = 0
    for (i in 0 ..< list1.size) {
        diff += abs(list1[i] - list2[i])
    }

    println(diff)
}

fun part2() {
    val lines = File("../input/01.txt").readLines()
    val map1 = mutableMapOf<Int, Int>()
    val map2 = mutableMapOf<Int, Int>()

    for (line in lines) {
        val numbers = line.split("\\s+".toRegex())
        val n1 = numbers[0].toInt()
        val n2 = numbers[1].toInt()
        if (n1 in map1) {
            map1[n1] = map1[n1]!! + 1
        } else {
            map1[n1] = 1
        }

        if (n2 in map2) {
            map2[n2] = map2[n2]!! + 1
        } else {
            map2[n2] = 1
        }
    }

    var simScore = 0
    for ((number, times) in map1) {
        simScore += times * number * map2.getOrDefault(number, 0)
    }
    println(simScore)
}