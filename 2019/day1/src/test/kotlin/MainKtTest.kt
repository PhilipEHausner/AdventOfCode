import kotlin.test.Test
import kotlin.test.assertEquals

internal class MainKtTest {

    @Test
    fun testComputeFuel() {
        assertEquals(computeFuel(12), 2)
        assertEquals(computeFuel(14), 2)
        assertEquals(computeFuel(1969), 654)
        assertEquals(computeFuel(100756), 33583)
    }
}