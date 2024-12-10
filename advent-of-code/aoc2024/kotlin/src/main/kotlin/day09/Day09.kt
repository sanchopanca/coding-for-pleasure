package engineer.kovalev.day09

import java.io.File
import java.util.PriorityQueue

data class Segment(val from: Int, val to: Int): Comparable<Segment> {
    override fun compareTo(other: Segment): Int {
        return this.from.compareTo(other.from)
    }

    fun fitsIn(other: Segment): Boolean {
        return this.len() <= other.len()
    }

    fun len(): Int {
        return this.to - this.from
    }
}
data class FileOnDisk(val segment: Segment, val id: Int): Comparable<FileOnDisk> {
    override fun compareTo(other: FileOnDisk): Int {
        return this.segment.compareTo(other.segment)
    }

    fun relocate(hole: Segment): Pair<FileOnDisk, Segment?> {
        if (!this.segment.fitsIn(hole)) {
            throw Exception("bad invocation of relocate()")
        }

        val newFile = FileOnDisk(Segment(hole.from, hole.from + this.segment.len()), this.id)

        var newHole: Segment? = null

        if (this.segment.len() != hole.len()) {
            newHole = Segment(newFile.segment.to, hole.to)
        }

        return Pair(newFile, newHole)
    }
}

fun main() {
    part1()
    part2()
}

fun part1() {
    val disk = getData()

    var left = 0
    var right = disk.size - 1

    while (left < right) {
        while (disk[left] != -1) {
            left++
        }
        while (disk[right] == -1) {
            right--
        }
        if (right <= left) {
            break
        }
        disk.swap(left, right)
    }

    println(disk.checksum())
}

fun part2() {
    val (files, holes) = getData2()

    val fileHeap = PriorityQueue(files)

    for (file in files.reversed()) {
        var relocateTo: Segment? = null
        for (hole in holes.sorted()) {  // duh!
            if (hole.from > file.segment.from) {
                break
            }
            if (file.segment.fitsIn(hole)) {
                relocateTo = hole
                break
            }
        }
        if (relocateTo != null) {
            val (newFile, newHole) = file.relocate(relocateTo)
            fileHeap.remove(file)
            fileHeap.add(newFile)

            holes.remove(relocateTo)
            if (newHole != null) {
                holes.add(newHole)
            }
        }
    }

    println(fileHeap.checksum())
}

fun getData(): MutableList<Int> {
    val map = File("../input/09.txt").readLines().first()

    val disk = mutableListOf<Int>()
    var readingFile = true
    for ((i, digit) in map.withIndex()) {
        val n = digit.toString().toInt()
        val data = if (readingFile) { i / 2 } else { -1 }
        for (j in 1..n) {
            disk.add(data)
        }
        readingFile = !readingFile
    }
    return disk
}

fun getData2(): Pair<MutableList<FileOnDisk>, PriorityQueue<Segment>> {
    val map = File("../input/09.txt").readLines().first()

    val files = mutableListOf<FileOnDisk>()
    val holes = PriorityQueue<Segment>()
    var readingFile = true
    var offset = 0
    for ((i, digit) in map.withIndex()) {
        val n = digit.toString().toInt()
        if (n != 0) {
            val segment = Segment(offset, offset + n)
            if (readingFile) {
                files.add(FileOnDisk(segment, i / 2))
            } else {
                holes.add(segment)
            }
        }
        offset += n
        readingFile = !readingFile
    }

    return Pair(files, holes)

}

fun <E> MutableList<E>.swap(i: Int, j: Int) {
    val tmp = this[i]
    this[i] = this[j]
    this[j] = tmp
}

fun MutableList<Int>.print() {
    for (e in this) {
        if (e == -1) {
            print('.')
        } else {
            print(e)
        }
    }
    println()
}

fun MutableList<Int>.checksum(): Long {
    var result: Long = 0
    for ((i, id) in this.withIndex()) {
        if (id == -1) {
            continue
        }
        result += i * id
    }
    return result
}

fun PriorityQueue<FileOnDisk>.checksum(): Long {
    var result: Long = 0
    for (file in this) {
        for (i in file.segment.from ..< file.segment.to) {
            result += i * file.id
        }
    }
    return result
}
