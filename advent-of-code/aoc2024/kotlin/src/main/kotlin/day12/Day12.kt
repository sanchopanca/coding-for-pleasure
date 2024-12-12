package engineer.kovalev.day12

import java.io.File
import java.util.ArrayDeque

val DIRS = listOf(
    Pair(-1, 0),
    Pair(0, 1),
    Pair(1, 0),
    Pair(0, -1),
)

data class Coords(val x: Int, val y: Int) {
    fun neighbors(): Set<Coords> {
        return DIRS.map {
            val (dx, dy) = it
            Coords(x + dx, y + dy)
        }.toSet()
    }

    fun neighbors(map: List<String>): Set<Coords> {
        return neighbors().filter { it.x in map.indices && it.y in map[0].indices }.toSet()
    }

    fun neighborsWithCorners(): Iterator<Triple<Coords, Coords, Coords>> {
        val dirsWithCorner = listOf(
            Triple(
                Pair(-1, 0),
                Pair(0, 1),
                Pair(-1, 1),
            ),
            Triple(
                Pair(0, 1),
                Pair(1, 0),
                Pair(1, 1),
            ),
            Triple(
                Pair(1, 0),
                Pair(0, -1),
                Pair(1, -1),
            ),
            Triple(
                Pair(0, -1),
                Pair(-1, 0),
                Pair(-1, -1)
            )
        )

        val res = mutableListOf<Triple<Coords, Coords, Coords>>()
        for ((d1, d2, d3) in dirsWithCorner) {
            res.add(Triple(
                Coords(x + d1.first, y + d1.second),
                Coords(x + d2.first, y + d2.second),
                Coords(x + d3.first, y + d3.second),
            ))
        }

        return res.iterator()
    }
}

class Plot(val plants: Set<Coords>) {
    private fun area(): Int {
        return plants.size
    }

    private fun perimeter(): Int {
        var res = 0
        for (plant in plants) {
            val neighbors = plant.neighbors()
            val outside = (neighbors - plants).size
            res += outside
        }
        return res
    }

    private fun corners(): Int {
        var additionalCorners = 0
        val set = mutableSetOf<Set<Coords>>()
        for (plant in plants) {
            for ((neighbor1, neighbor2, cornerNeighbor) in plant.neighborsWithCorners()) {
                if (neighbor1 !in plants && neighbor2 !in plants) {
                    if (cornerNeighbor in plants) {
                        additionalCorners++
                    } else {
                        set.add(setOf(plant, neighbor1, neighbor2, cornerNeighbor))
                    }
                } else if (neighbor1 in plants && neighbor2 in plants && cornerNeighbor !in plants) {
                    set.add(setOf(plant, neighbor1, neighbor2, cornerNeighbor))
                }
            }
        }
        return set.size + additionalCorners
    }

    fun price(): Int {
        return area() * perimeter()
    }

    fun bulkPrice(): Int {
        return area() * corners()
    }
}

fun floodFill(c: Coords, map: List<String>): Plot {
    val set = mutableSetOf<Coords>()
    val type = map.get(c)
    val stack = ArrayDeque<Coords>()
    stack.push(c)
    while (!stack.isEmpty()) {
        val next = stack.pop()
        if (map.get(next) != type) {
            continue
        }
        set.add(next)
        for (n in next.neighbors(map)) {
            if (n !in set) {
                stack.push(n)
            }
        }
    }
    return Plot(set)
}

fun main() {
    val garden = File("../input/12.txt").readLines()
    val visited = mutableSetOf<Coords>()
    val plots = mutableListOf<Plot>()
    for (x in garden.indices) {
        for (y in garden[0].indices) {
            val c = Coords(x, y)
            if (c in visited) {
                continue
            }
            val plot = floodFill(c, garden)
            visited.addAll(plot.plants)
            plots.add(plot)
        }
    }

    println(plots.sumOf {it.price()})

    println(plots.sumOf { it.bulkPrice() })
}

fun List<String>.get(c: Coords): Char {
    return this[c.x][c.y]
}
