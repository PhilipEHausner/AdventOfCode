import kotlin.test.Test
import kotlin.test.assertEquals

internal class ShipComputerTest {
    private fun stringToIntList(string: String): List<Int> {
        return string.split(",").map { it.toInt() }
    }

    @Test
    fun testComputeProgramAlarm() {
        val tests = listOf(
            Pair(2, "1,0,0,0,99"),
            Pair(30, "1,1,1,4,99,5,6,0,99"),
            Pair(3500, "1,9,10,3,2,3,11,0,99,30,40,50")
        )
        for (test in tests) {
            val computer = ShipComputer(stringToIntList(test.second))
            assertEquals(test.first, computer.parse())
        }
    }
}