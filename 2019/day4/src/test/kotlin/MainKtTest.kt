import kotlin.test.Test
import kotlin.test.assertEquals

internal class MainKtTest {

    private fun stringToIntList(string: String): List<Int> {
        return string.split(",").map { it.toInt() }
    }

    @Test
    fun testIsValidPassword() {
        assertEquals(true, isValidPassword(111111))
        assertEquals(false, isValidPassword(223450))
        assertEquals(false, isValidPassword(123789))
    }

}