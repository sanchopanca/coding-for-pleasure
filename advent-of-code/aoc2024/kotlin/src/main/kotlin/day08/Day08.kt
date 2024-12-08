package engineer.kovalev.day08

import java.io.File

fun main() {
    part1()
    part2()
}

data class Location(val x: Int, val y: Int)

fun part1() {
    val map = File("../input/08.txt").readLines()

    val antennas = mutableMapOf<Char, MutableList<Location>>()

    for (x in map.indices) {
        for (y in map[x].indices) {
            val c = map[x][y]
            if (c != '.') {
                if (antennas.contains(c)) {
                    antennas[c]!!.add(Location(x, y))
                } else {
                    antennas[c] = mutableListOf(Location(x, y))
                }
            }
        }
    }

    val antinodes = mutableSetOf<Location>()
    for ((c, locations) in antennas) {
        for (i in locations.indices) {
            for (j in i+1..<locations.size) {
                val (x1, y1) = locations[i]
                val (x2, y2) = locations[j]
                val dx = x1 - x2
                val dy = y1 - y2

                val potentialLocations = mutableListOf<Location>()
                potentialLocations.add(Location(x1 + dx, y1 + dy))
                potentialLocations.add(Location(x2 - dx, y2 - dy))
                for (l in potentialLocations) {
                    if (l.x in map.indices && l.y in map[0].indices) {
                        antinodes.add(l)
                    }
                }
            }
        }
    }
    println(antinodes.size)
}

fun part2() {
    val map = File("../input/08.txt").readLines()

    val antennas = mutableMapOf<Char, MutableList<Location>>()

    for (x in map.indices) {
        for (y in map[x].indices) {
            val c = map[x][y]
            if (c != '.') {
                if (antennas.contains(c)) {
                    antennas[c]!!.add(Location(x, y))
                } else {
                    antennas[c] = mutableListOf(Location(x, y))
                }
            }
        }
    }

    val antinodes = mutableSetOf<Location>()
    for ((c, locations) in antennas) {
        for (i in locations.indices) {
            for (j in i+1..<locations.size) {
                var (x1, y1) = locations[i]
                var (x2, y2) = locations[j]
                val dx = x1 - x2
                val dy = y1 - y2

                while (x1 in map.indices && y1 in map[0].indices) {
                    antinodes.add(Location(x1, y1))
                    x1 += dx
                    y1 += dy
                }

                while (x2 in map.indices && y2 in map[0].indices) {
                    antinodes.add(Location(x2, y2))
                    x2 -= dx
                    y2 -= dy
                }
            }
        }
    }
    println(antinodes.size)
}