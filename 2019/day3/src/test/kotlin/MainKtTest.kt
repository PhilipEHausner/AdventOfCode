import kotlin.test.Test
import kotlin.test.assertEquals

internal class MainKtTest {

    private fun stringToIntList(string: String): List<Int> {
        return string.split(",").map { it.toInt() }
    }

    @Test
    fun testComputeProgramAlarm() {
        assertEquals(
            6,
            getMinimalManhattanDistanceOfIntersections(
                Pair(
                    stringToMovements("R8,U5,L5,D3"),
                    stringToMovements("U7,R6,D4,L4")
                )
            )
        )
        assertEquals(
            159,
            getMinimalManhattanDistanceOfIntersections(
                Pair(
                    stringToMovements("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                    stringToMovements("U62,R66,U55,R34,D71,R55,D58,R83")
                )
            )
        )
        assertEquals(
            135,
            getMinimalManhattanDistanceOfIntersections(
                Pair(
                    stringToMovements("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                    stringToMovements("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
                )
            )
        )
    }

}