package engineer.kovalev.day23

import java.io.File

fun main() {
    val content = File("../input/23.txt").readText()

    val G = mutableMapOf<String, MutableList<String>>()
    for (line in content.trim().lines()) {
        val (from, to) = line.split('-')
        G.getOrPut(from) { mutableListOf() }.add(to)
        G.getOrPut(to) { mutableListOf() }.add(from)
    }

    val triples = mutableSetOf<Set<String>>()
    loop@for ((node, connections) in G) {
        if (connections.size < 2) continue

        for (computer in connections) {
            val secondDegreeConnections = G[computer]!!
            for (sdc in secondDegreeConnections) {
                if (node in G[sdc]!!) {
                    triples.add(setOf(node, computer, sdc))
                }
            }
        }
    }

    println(triples.filter { it.any { it.startsWith('t') } }.size)

    println(G.values.map { it.size }.sorted())
    println(G.size)
    val largestClique = largestClique(G)
    println(largestClique.sorted().joinToString(separator = ","))
}

fun bronKerboschWithPivot(
    graph: Map<String, List<String>>,
    r: Set<String> = emptySet(),
    p: Set<String> = graph.keys,
    x: Set<String> = emptySet(),
    cliques: MutableList<Set<String>> = mutableListOf()
) {
    if (p.isEmpty() && x.isEmpty()) {
        cliques.add(r) // Maximal clique found
        return
    }

    // Choose a pivot (node with the most neighbors in P ∪ X)
    val pivot = (p + x).maxByOrNull { graph[it]?.size ?: 0 }

    // Nodes to explore: P \ neighbors(pivot)
    val nodesToExplore = p - (graph[pivot]?.toSet() ?: emptySet()).toSet()

    for (node in nodesToExplore) {
        val neighbors = graph[node]?.toSet() ?: emptySet()
        bronKerboschWithPivot(
            graph,
            r = r + node,
            p = p intersect neighbors,
            x = x intersect neighbors,
            cliques = cliques
        )
    }
}

fun largestClique(graph: Map<String, List<String>>): Set<String> {
    val cliques = mutableListOf<Set<String>>()
    bronKerboschWithPivot(graph, cliques = cliques)
    return cliques.maxByOrNull { it.size } ?: emptySet()
}