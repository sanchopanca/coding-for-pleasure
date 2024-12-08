package engineer.kovalev.day08

import java.io.File

fun main() {
    val map = File("../input/08.txt").readLines()
    part1(map)
    part2(map)
}

data class Location(val x: Int, val y: Int)

fun getAntennas(map: List<String>): Map<Char, MutableList<Location>> {
    val antennas = mutableMapOf<Char, MutableList<Location>>()

    for (x in map.indices) {
        for (y in map[x].indices) {
            val c = map[x][y]
            if (c != '.') {
                antennas.computeIfAbsent(c) { mutableListOf() }.add(Location(x, y))
            }
        }
    }
    return antennas
}

fun part1(map: List<String>) {
    val antennas = getAntennas(map)

    val antinodes = mutableSetOf<Location>()
    for ((_, locations) in antennas) {
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

fun part2(map: List<String>) {
    val antennas = getAntennas(map)

    val antinodes = mutableSetOf<Location>()
    for ((_, locations) in antennas) {
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