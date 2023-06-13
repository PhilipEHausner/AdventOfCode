import java.io.File
import java.util.LinkedList
import java.util.Queue

typealias Node = String

class Graph {
    private val nodes = mutableSetOf<Node>()
    private val edges = mutableMapOf<Node, MutableList<Node>>()
    private val centerOfMass: Node = "COM"

    fun addNode(node: Node) {
        this.nodes.add(node)
    }

    fun addEdge(source: Node, target: Node) {
        assert(source in this.nodes)
        assert(target in this.nodes)
        if (source !in this.edges.keys) {
            this.edges[source] = mutableListOf()
        }
        if (target !in this.edges.keys) {
            this.edges[target] = mutableListOf()
        }
        this.edges[source]!!.add(target)
        this.edges[target]!!.add(source)
    }

    fun addOrbit(orbit: String) {
        val objects = orbit.split(')')
        assert(objects.size == 2)
        this.addNode(objects[0])
        this.addNode(objects[1])
        this.addEdge(objects[0], objects[1])
    }

    fun addOrbits(orbits: List<String>) {
        for (orbit in orbits) {
            this.addOrbit(orbit)
        }
    }

    fun computeOrbitCountChecksum(
        obj: Node = this.centerOfMass,
        currentChecksum: Int = 0,
        visited: MutableSet<Node> = mutableSetOf()
    ): Int {
        visited.add(obj)
        if (obj !in this.edges.keys) {
            return currentChecksum
        }
        var checksum = currentChecksum
        for (orbit in this.edges[obj]!!) {
            if (orbit in visited) {
                continue
            }
            checksum += this.computeOrbitCountChecksum(orbit, currentChecksum + 1, visited)
        }
        return checksum
    }

    fun stepsBetweenObjects(source: Node, target: Node): Int {
        val visited = mutableSetOf<Node>()
        val queue: Queue<Pair<String, Int>> = LinkedList(listOf(Pair(source, 0)))

        while (queue.size > 0) {
            val (node, range) = queue.remove()
            if (node == target) return range

            visited.add(node)
            for (nextNode in this.edges[node]!!) {
                if (nextNode in visited) continue
                queue.add(Pair(nextNode, range + 1))
            }
        }

        throw Error("There is no connection between nodes $source and ${target}.")
    }
}

fun readFile(file: String): List<String> {
    return File(file).readLines()
}

fun part1(orbits: List<String>) {
    val graph = Graph()
    graph.addOrbits(orbits)
    val checksum = graph.computeOrbitCountChecksum()
    println("Solution part 1: ${checksum}.")
}

fun part2(orbits: List<String>) {
    val graph = Graph()
    graph.addOrbits(orbits)
    val steps = graph.stepsBetweenObjects("YMQ", "HZ2")
    println("Solution part 2: ${steps}.")
}

fun main() {
    val orbits = readFile("files/day6.txt")
    part1(orbits)
    part2(orbits)
}