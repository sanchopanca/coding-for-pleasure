package engineer.kovalev.day16

import java.io.File
import java.util.PriorityQueue

fun main() {
    part1()
}

fun part1() {
    val map = File("../input/16.txt").readLines()
    var (sX, sY) = 0 to 0
    var (eX, eY) = 0 to 0
    for ((y, line) in map.withIndex()) {
        for ((x, c) in line.withIndex()) {
            if (c == 'S') {
                sX = x
                sY = y
            }
            if (c == 'E') {
                eX = x
                eY = y
            }
        }
    }

    val result = findShortestPath(map, sX, sY, eX, eY)
    println("Shortest path score: $result")
}

fun findShortestPath(map: List<String>, startX: Int, startY: Int, endX: Int, endY: Int): Int {
    val directions = listOf(
        Pair(0, -1), // North
        Pair(1, 0),  // East
        Pair(0, 1),  // South
        Pair(-1, 0)  // West
    )

    data class State(val x: Int, val y: Int, val direction: Int, val score: Int)

    val rows = map.size
    val cols = map[0].length
    val visited = Array(rows) { Array(cols) { BooleanArray(4) } }

    val priorityQueue = PriorityQueue<State>(compareBy { it.score })
    priorityQueue.add(State(startX, startY, 1, 0)) // Start facing East

    while (priorityQueue.isNotEmpty()) {
        val current = priorityQueue.poll()

        if (current.x == endX && current.y == endY) {
            return current.score // Reached the destination
        }

        if (visited[current.y][current.x][current.direction]) continue
        visited[current.y][current.x][current.direction] = true

        // Move forward
        val (dx, dy) = directions[current.direction]
        val nextX = current.x + dx
        val nextY = current.y + dy
        if (nextX in 0 until cols && nextY in 0 until rows && map[nextY][nextX] != '#') {
            priorityQueue.add(State(nextX, nextY, current.direction, current.score + 1))
        }

        // Rotate clockwise
        val clockwiseDir = (current.direction + 1) % 4
        if (!visited[current.y][current.x][clockwiseDir]) {
            priorityQueue.add(State(current.x, current.y, clockwiseDir, current.score + 1000))
        }

        // Rotate counterclockwise
        val counterClockwiseDir = (current.direction + 3) % 4
        if (!visited[current.y][current.x][counterClockwiseDir]) {
            priorityQueue.add(State(current.x, current.y, counterClockwiseDir, current.score + 1000))
        }
    }

    return -1 // No path found
}
