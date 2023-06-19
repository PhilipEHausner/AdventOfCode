import aoc.Image
import kotlin.test.Test
import kotlin.test.assertEquals

internal class ImageTest {
    private fun stringToIntList(string: String): List<Int> {
        return string.toCharArray().map { it.digitToInt() }
    }

    @Test
    fun testChecksum() {
        val encoding = stringToIntList("122110001012")
        val imageSize = Pair(3, 2)
        val image = Image(encoding, imageSize)
        assertEquals(6, image.checkSum())
    }

    @Test
    fun testPart1() {
        val encoding = readFile("./files/day8.txt")
        val imageSize = Pair(25, 6)
        val image = Image(encoding, imageSize)
        assertEquals(1950, image.checkSum())
    }
}