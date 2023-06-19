import java.io.File
import java.nio.charset.Charset


fun readFile(file: String): List<Int> {
    return File(file).readText(Charset.defaultCharset()).toCharArray().filter { !it.isWhitespace() }
        .map { it.digitToInt() }
}

fun part1(encoding: List<Int>, imageSize: Pair<Int, Int>) {
    val image = Image(encoding, imageSize)
    println("Solution part 1: ${image.checkSum()}.")
}

fun main() {
    val encoding = readFile("./files/day8.txt")
    val imageSize = Pair(25, 6)
    part1(encoding, imageSize)
}