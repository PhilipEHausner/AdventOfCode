import kotlin.test.Test
import kotlin.test.assertEquals

internal class MainKtTest {

    @Test
    fun testComputeFuel() {
        assertEquals(2, computeFuel(12))
        assertEquals(2, computeFuel(14))
        assertEquals(654, computeFuel(1969))
        assertEquals(33583, computeFuel(100756))
    }

    @Test
    fun testComputeTotalFuel() {
        assertEquals(2, computeTotalFuel(14))
        assertEquals(966, computeTotalFuel(1969))
        assertEquals(50346, computeTotalFuel(100756))
    }
}