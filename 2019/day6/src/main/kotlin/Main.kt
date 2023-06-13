import java.io.File

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
        this.edges[source]!!.add(target)
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

    fun computeOrbitCountChecksum(obj: Node = this.centerOfMass, currentChecksum: Int = 0): Int {
        if (obj !in this.edges.keys) {
            return currentChecksum
        }
        var checksum = currentChecksum
        for (orbit in this.edges[obj]!!) {
            checksum += this.computeOrbitCountChecksum(orbit, currentChecksum + 1)
        }
        return checksum
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

fun main() {
    val orbits = readFile("files/day6.txt")
    part1(orbits)
}