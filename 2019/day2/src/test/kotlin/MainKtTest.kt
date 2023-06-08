import kotlin.test.Test
import kotlin.test.assertEquals

internal class MainKtTest {

    private fun stringToIntList(string: String): List<Int> {
        return string.split(",").map { it.toInt() }
    }

    @Test
    fun testComputeProgramAlarm() {
        assertEquals(2, computeProgramAlarm(stringToIntList("1,0,0,0,99")))
        assertEquals(30, computeProgramAlarm(stringToIntList("1,1,1,4,99,5,6,0,99")))
        assertEquals(3500, computeProgramAlarm(stringToIntList("1,9,10,3,2,3,11,0,99,30,40,50")))
    }

}