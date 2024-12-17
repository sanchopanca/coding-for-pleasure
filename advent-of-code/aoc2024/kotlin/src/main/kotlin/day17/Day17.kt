package engineer.kovalev.day17

import java.io.File

const val ADV = 0L
const val BXL = 1L
const val BST = 2L
const val JNZ = 3L
const val BXC = 4L
const val OUT = 5L
const val BDV = 6L
const val CDV = 7L


class VM(var a: Long, val ram: List<Long>) {
    var b = 0L
    var c = 0L
    var ip = 0

    fun run(): List<Long> {
        val result = mutableListOf<Long>()
        while (ip < ram.size - 1) {
            val (instruction, operand) = ram[ip] to ram[ip + 1]
            when (instruction) {
                ADV -> {
                    a /= 1L shl combo(operand).toInt()
                }
                BXL -> {
                    b = b xor operand
                }
                BST -> {
                    b = combo(operand) % 8
                }
                JNZ -> {
                    if (a != 0L) {
                        ip = operand.toInt()
                        continue
                    }
                }
                BXC -> {
                    b = b xor c
                }
                OUT -> {
                    val v = combo(operand) % 8
                    result.add(v)
                }
                BDV -> {
                    b = a / (1 shl combo(operand).toInt())
                }
                CDV -> {
                    c = a / (1 shl combo(operand).toInt())
                }
                else -> {
                    println("Seen $instruction")
                }
            }
            ip += 2
        }
        return result
    }

    private fun combo(v: Long): Long {
        if (v in 0..3) {
            return v
        }
        return when (v) {
            4L -> a
            5L -> b
            6L -> c
            else -> {
                throw Exception("unreachable")
            }
        }
    }
}

fun find(prefix: String, ram: List<Long>): List<String> {
    if (prefix.length == ram.size) {
        return listOf(prefix)
    }

    val results = mutableListOf<String>()
    for (d in '0'..'7') {
        val a = (prefix + d).toLong(8)
        val vm = VM(a, ram)
        val output = vm.run()
        if (output[output.lastIndex - prefix.length] == ram[ram.lastIndex - prefix.length]) {
            results.addAll(find(prefix + d, ram))
        }
    }
    return results
}


fun main() {
    val input = File("../input/17.txt").readText()
    val (registers, program) = input.split("\n\n")
    val a = registers.split('\n').first().split(' ').last().toLong()
    val ram = program.split(' ').last().split(',').map { it.trim().toLong() }.toList()

    val vm = VM(a, ram)
    println(vm.run().joinToString(","))

    println(find("", ram).map { it.toLong(8) }.min())
}