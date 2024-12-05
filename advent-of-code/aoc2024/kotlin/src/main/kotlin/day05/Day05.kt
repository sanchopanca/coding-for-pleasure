package engineer.kovalev.day05

import java.io.File

class Rule(val before: Int, val after: Int)

fun main() {
    val (rulesStr, printJobsStr) = File("../input/05.txt").readText().trim().split("\n\n")

    val rules = mutableListOf<Rule>()
    for (line in rulesStr.lines()) {
        val (before, after) = line.split('|').map { it.toInt() }
        rules.add(Rule(before, after))
    }

    var result = 0
    var result2 = 0

    loop@ for (line in printJobsStr.lines()) {
        val jobAsList = line.split(',').map { it.toInt() }
        val job = toMap(jobAsList)

        for (rule in rules) {
            if (!ruleHolds(job, rule)) {
                val reorderedJobs = reorder(jobAsList, rules)
                result2 += reorderedJobs[reorderedJobs.size / 2]

                continue@loop
            }
        }
        result += jobAsList[jobAsList.size / 2]
    }

    println(result)
    println(result2)
}

fun reorder(job: List<Int>, rules: List<Rule>): List<Int> {
    val newJob = job.toMutableList()
    val newJobMap = toMap(newJob).toMutableMap()
    for (i in 1..job.size) {  // just my intuition that this should be enough
        for (rule in rules) {
            if (!ruleHolds(newJobMap, rule)) {
                newJob.swap(newJobMap[rule.before]!!, newJobMap[rule.after]!!)
                newJobMap.swap(rule.before, rule.after)
            }
        }
    }
    return newJob
}

fun toMap(list: List<Int>): Map<Int, Int> = list.withIndex().associate { it.value to it.index }

fun ruleHolds(job: Map<Int, Int>, rule: Rule): Boolean {
    val containsBoth = job.contains(rule.before) && job.contains(rule.after)
    return !(containsBoth && job[rule.before]!! > job[rule.after]!!)
}

fun <T> MutableList<T>.swap(index1: Int, index2: Int){
    val tmp = this[index1]
    this[index1] = this[index2]
    this[index2] = tmp
}

fun <K, V> MutableMap<K, V>.swap(k1: K, k2: K){
    val tmp = this[k1]!!
    this[k1] = this[k2]!!
    this[k2] = tmp
}
