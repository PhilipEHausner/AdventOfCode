import java.io.File
import java.nio.charset.Charset

fun main() {
    val intcode = readFile("./files/day9.txt")
    part1(intcode)
}

fun part1(intcode: List<Long>) {
    val shipComputer = IntcodeComputer(intcode)
    shipComputer.pushInput(1)
    val result = shipComputer.process()
    println("Result of part 1: $result.")
}

fun readFile(file: String): List<Long> {
    return File(file).readText(Charset.defaultCharset()).filter { !it.isWhitespace() }.split(",").map { it.toLong() }
}

