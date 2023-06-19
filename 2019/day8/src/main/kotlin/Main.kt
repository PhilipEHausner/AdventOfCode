import aoc.Color
import aoc.Image
import korlibs.image.bitmap.Bitmap32
import korlibs.image.color.Colors
import korlibs.image.color.RGBA
import korlibs.image.format.ImageData
import korlibs.image.format.showImagesAndWait
import java.io.File
import java.nio.charset.Charset


fun readFile(file: String): List<Int> {
    return File(file).readText(Charset.defaultCharset()).toCharArray().filter { !it.isWhitespace() }
        .map { it.digitToInt() }
}

fun aocColorToRGBA(color: aoc.Color): RGBA {
    return when(color) {
        Color.BLACK -> Colors.BLACK
        Color.WHITE -> Colors.WHITE
        Color.TRANSPARENT -> Colors.TRANSPARENT
    }

}

fun part1(encoding: List<Int>, imageSize: Pair<Int, Int>) {
    val image = Image(encoding, imageSize)
    println("Solution part 1: ${image.checkSum()}.")
}

suspend fun part2(encoding: List<Int>, imageSize: Pair<Int, Int>) {
    val image = Image(encoding, imageSize)
    val decodedImage = image.decodeImage()
    val bitmap = Bitmap32(imageSize.first, imageSize.second) { x, y -> aocColorToRGBA(decodedImage[y][x]) }
    println("Solution part 2:")
    ImageData(bitmap).showImagesAndWait(0)
}

suspend fun main() {
    val encoding = readFile("./files/day8.txt")
    val imageSize = Pair(25, 6)
    part1(encoding, imageSize)
    part2(encoding, imageSize)
}