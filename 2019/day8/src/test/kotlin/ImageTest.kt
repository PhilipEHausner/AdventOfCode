import kotlin.math.max
import kotlin.test.Test
import kotlin.test.assertEquals

internal class ImageTest {
    private fun stringToIntList(string: String): List<Int> {
        return string.toCharArray().map { it.digitToInt() }
    }

    @Test
    fun testChecksum() {
        val encoding = stringToIntList("122116789012")
        val imageSize = Pair(3, 2)
        val image = Image(encoding, imageSize)
        assertEquals(6, image.checkSum())
    }

    @Test
    fun testPart1() {
    }
}