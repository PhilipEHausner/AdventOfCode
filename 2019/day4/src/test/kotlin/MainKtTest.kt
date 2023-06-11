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
        assertEquals(true, isValidPassword(112233, true))
        assertEquals(false, isValidPassword(123444, true))
        assertEquals(true, isValidPassword(111122, true))
    }

}