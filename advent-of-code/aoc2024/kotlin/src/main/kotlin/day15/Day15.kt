package engineer.kovalev.day15

import java.io.File

enum class Object {
    Empty,
    Wall,
    Box,
}

enum class Object2 {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

enum class Direction(val vec: Pair<Int, Int>) {
    Up(Pair(0, -1)),
    Right(Pair(1, 0)),
    Down(Pair(0, 1)),
    Left(Pair(-1, 0)),
}

class Warehouse(lines: List<String>) {
    val floor: MutableList<MutableList<Object>> = mutableListOf()
    var x: Int = 0
    var y: Int = 0

    init {
        for ((i, line) in lines.withIndex()) {
            this.floor.add(mutableListOf())
            for ((j, c) in line.withIndex()) {
                val type = when (c) {
                    '#' -> {
                        Object.Wall
                    }
                    'O' -> {
                        Object.Box
                    }
                    '.' -> {
                        Object.Empty
                    }
                    else -> { // '@'
                        this.x = j
                        this.y = i
                        Object.Empty
                    }
                }
                this.floor[i].add(type)
            }
        }
    }

    fun move(direction: Direction) {
        val (dx, dy) = direction.vec
        val (nX, nY) = x + dx to y + dy
        val o = floor[nY][nX]
        when (o) {
            Object.Empty -> {
                x = nX
                y = nY
            }
            Object.Wall -> {
                return
            }
            Object.Box -> {
                if (moveBox(nX, nY, direction)) {
                    floor[nY][nX] = Object.Empty
                    x = nX
                    y = nY
                }
            }
        }
    }

    fun gps(): Int {
        var result = 0

        for (y in floor.indices) {
            for (x in floor[y].indices) {
                if (floor[y][x] == Object.Box) {
                    result += x + 100 * y
                }
            }
        }

        return result
    }

    fun print() {
        for (line in floor) {
            println(line.map { when (it) {
                Object.Empty -> '.'
                Object.Wall -> '#'
                Object.Box -> 'O'
            } }.joinToString(""))
        }
    }

    private fun moveBox(fromX: Int, fromY: Int, direction: Direction): Boolean {
        val (dx, dy) = direction.vec
        val (nX, nY) = fromX + dx to fromY + dy
        val o = floor[nY][nX]
        return when (o) {
            Object.Empty -> {
                floor[nY][nX] = Object.Box
                true
            }
            Object.Wall -> false
            Object.Box -> {
                if (moveBox(nX, nY, direction)) {
                    floor[nY][nX] = Object.Box
                    true
                } else {
                    false
                }
            }
        }
    }
}

class Warehouse2(lines: List<String>) {
    val floor: MutableList<MutableList<Object2>> = mutableListOf()
    var x: Int = 0
    var y: Int = 0

    init {
        for ((i, line) in lines.withIndex()) {
            this.floor.add(mutableListOf())
            val wideLine = line
                .replace("#", "##")
                .replace(".", "..")
                .replace("O", "[]")
                .replace("@", "@.")
            for ((j, c) in wideLine.withIndex()) {
                val type = when (c) {
                    '#' -> Object2.Wall
                    '[' -> Object2.BoxLeft
                    ']' -> Object2.BoxRight
                    '.' -> Object2.Empty
                    else -> { // '@'
                        this.x = j
                        this.y = i
                        Object2.Empty
                    }
                }
                this.floor[i].add(type)
            }
        }
    }

    fun move(direction: Direction) {
        val (dx, dy) = direction.vec
        val (nX, nY) = x + dx to y + dy
        val o = floor[nY][nX]
        when (o) {
            Object2.Empty -> {
                x = nX
                y = nY
            }
            Object2.Wall -> {
                return
            }
            Object2.BoxLeft -> {
                if (canMove(nX, nY, direction)) {
                    moveBox(nX, nY, direction)
                    floor[nY][nX] = Object2.Empty
                    x = nX
                    y = nY
                    if (direction == Direction.Up || direction == Direction.Down) {
                        floor[nY][nX + 1] = Object2.Empty
                    }
                }
            }
            Object2.BoxRight -> {
                if (canMove(nX - 1, nY, direction)) {
                    moveBox(nX - 1, nY, direction)
                    floor[nY][nX] = Object2.Empty
                    x = nX
                    y = nY
                    if (direction == Direction.Up || direction == Direction.Down) {
                        floor[nY][nX - 1] = Object2.Empty
                    }
                }
            }
        }
    }

    fun gps(): Int {
        var result = 0

        for (y in floor.indices) {
            for (x in floor[y].indices) {
                if (floor[y][x] == Object2.BoxLeft) {
                    result += x + 100 * y
                }
            }
        }

        return result
    }

    fun print() {
        for ((y, line) in floor.withIndex()) {
            val printLine = line.map { when (it) {
                Object2.Empty -> '.'
                Object2.Wall -> '#'
                Object2.BoxLeft -> '['
                Object2.BoxRight -> ']'
            } }.joinToString("")
            for ((x, c) in printLine.withIndex()) {
                if (x == this.x && y == this.y) {
                    if (this.floor[y][x] != Object2.Empty) {
                        throw Exception ("where i'm standing?")
                    }
                    print('@')
                } else {
                    print(c)
                }
            }
            println()
        }
        println()
    }

    private fun moveBox(fromX: Int, fromY: Int, direction: Direction): Boolean {
        when (direction) {
            Direction.Right -> {
                when (floor[fromY][fromX + 2]) {
                    Object2.Empty -> {
                        floor[fromY][fromX + 2] = Object2.BoxRight
                        floor[fromY][fromX + 1] = Object2.BoxLeft
                        return true
                    }
                    Object2.Wall -> return false
                    Object2.BoxLeft -> {
                        if (moveBox(fromX + 2, fromY, direction)) {
                            floor[fromY][fromX + 2] = Object2.BoxRight
                            floor[fromY][fromX + 1] = Object2.BoxLeft
                            return true
                        }
                    }
                    Object2.BoxRight -> throw Exception("unreachable")
                }
            }
            Direction.Left -> {
                when (floor[fromY][fromX - 1]) {
                    Object2.Empty -> {
                        floor[fromY][fromX - 1] = Object2.BoxLeft
                        floor[fromY][fromX] = Object2.BoxRight
                        return true
                    }
                    Object2.Wall -> return false
                    Object2.BoxRight -> {
                        if (moveBox(fromX - 2, fromY, direction)) {
                            floor[fromY][fromX - 1] = Object2.BoxLeft
                            floor[fromY][fromX] = Object2.BoxRight
                            return true
                        }
                    }
                    Object2.BoxLeft -> {
                        throw Exception("unreachable")
                    }
                }
            }
            else -> {
                val (_, dy) = direction.vec
                when (floor[fromY + dy][fromX]) {
                    Object2.Empty -> {
                        when (floor[fromY + dy][fromX + 1]) {
                            Object2.Empty -> {
                                floor[fromY + dy][fromX] = Object2.BoxLeft
                                floor[fromY + dy][fromX + 1] = Object2.BoxRight
                                return true
                            }
                            Object2.Wall -> return false
                            Object2.BoxLeft -> {
                                if (moveBox(fromX + 1, fromY + dy, direction)) {
                                    floor[fromY + dy][fromX] = Object2.BoxLeft
                                    floor[fromY + dy][fromX + 1] = Object2.BoxRight
                                    floor[fromY + dy][fromX + 2] = Object2.Empty
                                    return true
                                }
                            }
                            Object2.BoxRight -> throw Exception("unreachable")
                        }
                    }
                    Object2.Wall -> return false
                    Object2.BoxLeft -> {
                        if (moveBox(fromX, fromY + dy, direction)) {
                            floor[fromY + dy][fromX] = Object2.BoxLeft
                            floor[fromY + dy][fromX + 1] = Object2.BoxRight
                            return true
                        }
                    }
                    Object2.BoxRight -> {
                        if (floor[fromY + dy][fromX + 1] == Object2.Wall) {
                            return false
                        }
                        var movedOther = false
                        if (floor[fromY + dy][fromX + 1] == Object2.BoxLeft) {
                            if (!moveBox(fromX + 1, fromY + dy, direction)) {
                                return false
                            }
                            movedOther = true
                        }
                        if (moveBox(fromX - 1, fromY + dy, direction)) {
                            floor[fromY + dy][fromX] = Object2.BoxLeft
                            floor[fromY + dy][fromX + 1] = Object2.BoxRight
                            floor[fromY + dy][fromX - 1] = Object2.Empty // ???
                            if (movedOther) {
                                floor[fromY + dy][fromX + 2] = Object2.Empty
                            }
                            return true
                        }
                    }
                }
            }
        }
        return false
    }

    private fun canMove(fromX: Int, fromY: Int, direction: Direction): Boolean {
        when (direction) {
            Direction.Right -> {
                when (floor[fromY][fromX + 2]) {
                    Object2.Empty -> {

                        return true
                    }
                    Object2.Wall -> return false
                    Object2.BoxLeft -> {
                        if (canMove(fromX + 2, fromY, direction)) {
                            return true
                        }
                    }
                    Object2.BoxRight -> throw Exception("unreachable")
                }
            }
            Direction.Left -> {
                when (floor[fromY][fromX - 1]) {
                    Object2.Empty -> {
                        return true
                    }
                    Object2.Wall -> return false
                    Object2.BoxRight -> {
                        if (canMove(fromX - 2, fromY, direction)) {
                            return true
                        }
                    }
                    Object2.BoxLeft -> {
                        throw Exception("unreachable")
                    }
                }
            }
            else -> {
                val (_, dy) = direction.vec
                when (floor[fromY + dy][fromX]) {
                    Object2.Empty -> {
                        when (floor[fromY + dy][fromX + 1]) {
                            Object2.Empty -> {
                                return true
                            }
                            Object2.Wall -> return false
                            Object2.BoxLeft -> {
                                if (canMove(fromX + 1, fromY + dy, direction)) {
                                    return true
                                }
                            }
                            Object2.BoxRight -> throw Exception("unreachable")
                        }
                    }
                    Object2.Wall -> return false
                    Object2.BoxLeft -> {
                        if (canMove(fromX, fromY + dy, direction)) {
                            return true
                        }
                    }
                    Object2.BoxRight -> {
                        if (floor[fromY + dy][fromX + 1] == Object2.Wall) {
                            return false
                        }
                        if (floor[fromY + dy][fromX + 1] == Object2.BoxLeft) {
                            if (!canMove(fromX + 1, fromY + dy, direction)) {
                                return false
                            }
                        }
                        if (canMove(fromX - 1, fromY + dy, direction)) {
                            return true
                        }
                    }
                }
            }
        }
        return false
    }
}


fun main() {
    part1()
    part2()
}

fun part1() {
    val content = File("../input/15.txt").readText()
    val (floor, movements) = content.split("\n\n")
    val warehouse = Warehouse(floor.split('\n'))
    for (m in movements) {
        when (m) {
            '^' -> warehouse.move(Direction.Up)
            '>' -> warehouse.move(Direction.Right)
            'v' -> warehouse.move(Direction.Down)
            '<' -> warehouse.move(Direction.Left)
        }
    }
    println(warehouse.gps())
}

fun part2() {
    val content = File("../input/15.txt").readText()
    val (floor, movements) = content.split("\n\n")
    val warehouse = Warehouse2(floor.split('\n'))
    for (m in movements) {
        if (m == '\n') continue
        when (m) {
            '^' -> warehouse.move(Direction.Up)
            '>' -> warehouse.move(Direction.Right)
            'v' -> warehouse.move(Direction.Down)
            '<' -> warehouse.move(Direction.Left)
        }
    }
    println(warehouse.gps())

}
