import kotlin.test.Test
import kotlin.test.assertEquals

internal class GraphTest {
    private val testOrbits = listOf<String>(
        "COM)B",
        "B)C",
        "C)D",
        "D)E",
        "E)F",
        "B)G",
        "G)H",
        "D)I",
        "E)J",
        "J)K",
        "K)L",
    )

    @Test
    fun testOrbitCountChecksum() {
        val graph = Graph()
        graph.addOrbits(this.testOrbits)
        val checksum = graph.computeOrbitCountChecksum()
        assertEquals(42, checksum)
    }

    @Test
    fun testPart1() {
        val orbits = readFile("files/day6.txt")
        val graph = Graph()
        graph.addOrbits(orbits)
        val checksum = graph.computeOrbitCountChecksum()
        assertEquals(295936, checksum)
    }

    @Test
    fun testStepsBetweenObjects() {
        val graph = Graph()
        graph.addOrbits(this.testOrbits)
        val steps = graph.stepsBetweenObjects("K", "I")
        assertEquals(4, steps)
    }

    @Test
    fun testPart2() {
        val orbits = readFile("files/day6.txt")
        val graph = Graph()
        graph.addOrbits(orbits)
        val steps = graph.stepsBetweenObjects("YMQ", "HZ2")
        assertEquals(457, steps)
    }
}