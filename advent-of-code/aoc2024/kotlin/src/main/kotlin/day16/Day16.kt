package engineer.kovalev.day16

import java.io.File
import java.util.PriorityQueue

fun main() {
    part1()
    part2()
}

fun part1() {
    val map = File("../input/16.txt").readLines()
    val (start, end) = findStartAndEnd(map)

    val result = dijkstraForShortestPath(map, start.first, start.second, end.first, end.second)
    println("Shortest path score: $result.first")
}

fun part2() {
    val map = File("../input/16.txt").readLines()
    val (start, end) = findStartAndEnd(map)

    val (_, paths) = dijkstraForShortestPath(map, start.first, start.second, end.first, end.second)
    val uniqueTiles = paths.flatten().toSet().size
    println("Total unique tiles visited by all shortest paths: $uniqueTiles")
}

fun findStartAndEnd(map: List<String>): Pair<Pair<Int, Int>, Pair<Int, Int>> {
    var start = 0 to 0
    var end = 0 to 0

    for ((y, line) in map.withIndex()) {
        for ((x, c) in line.withIndex()) {
            when (c) {
                'S' -> start = x to y
                'E' -> end = x to y
            }
        }
    }

    return start to end
}

fun dijkstraForShortestPath(
    map: List<String>,
    startX: Int,
    startY: Int,
    endX: Int,
    endY: Int
): Pair<Int, List<Set<Pair<Int, Int>>>> {
    val directions = listOf(
        Pair(0, -1), // North
        Pair(1, 0),  // East
        Pair(0, 1),  // South
        Pair(-1, 0)  // West
    )

    data class State(val x: Int, val y: Int, val direction: Int, val score: Int, val path: Set<Pair<Int, Int>>)

    val rows = map.size
    val cols = map[0].length

    // Minimum scores to reach each state (x, y, direction)
    val minScores = Array(rows) { Array(cols) { IntArray(4) { Int.MAX_VALUE } } }
    val allPaths = mutableListOf<Set<Pair<Int, Int>>>()

    val priorityQueue = PriorityQueue<State>(compareBy { it.score })
    priorityQueue.add(State(startX, startY, 1, 0, setOf(startX to startY))) // Start facing East
    var shortestCost = Int.MAX_VALUE

    while (priorityQueue.isNotEmpty()) {
        val current = priorityQueue.poll()

        // Stop processing if the score exceeds the shortest cost found
        if (current.score > shortestCost) break

        // Update shortest cost if we reach the endpoint
        if (current.x == endX && current.y == endY) {
            shortestCost = current.score
            allPaths.add(current.path) // Add this path
            continue
        }

        // Skip if we've already found a cheaper path to this state
        if (current.score > minScores[current.y][current.x][current.direction]) continue

        // Update the minimum score for this state
        minScores[current.y][current.x][current.direction] = current.score

        // Explore all possible moves
        val (dx, dy) = directions[current.direction]
        val nextX = current.x + dx
        val nextY = current.y + dy

        // Move forward
        if (nextX in 0 until cols && nextY in 0 until rows && map[nextY][nextX] != '#') {
            priorityQueue.add(
                State(
                    nextX,
                    nextY,
                    current.direction,
                    current.score + 1,
                    current.path + (nextX to nextY)
                )
            )
        }

        // Rotate clockwise
        val clockwiseDir = (current.direction + 1) % 4
        if (current.score + 1000 <= shortestCost) {
            priorityQueue.add(
                State(
                    current.x,
                    current.y,
                    clockwiseDir,
                    current.score + 1000,
                    current.path
                )
            )
        }

        // Rotate counterclockwise
        val counterClockwiseDir = (current.direction + 3) % 4
        if (current.score + 1000 <= shortestCost) {
            priorityQueue.add(
                State(
                    current.x,
                    current.y,
                    counterClockwiseDir,
                    current.score + 1000,
                    current.path
                )
            )
        }
    }

    return shortestCost to allPaths
}
