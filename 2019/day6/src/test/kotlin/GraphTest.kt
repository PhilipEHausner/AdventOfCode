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

}