import java.io.File

fun readFile(file: String): List<Int> {
    return File(file).useLines { it.toList() }.map { it.toInt() }
}

fun computeFuel(mass: Int): Int {
    return (mass / 3) - 2
}

fun part1(numbers: List<Int>) {
    var fuel = 0;
    numbers.forEach { fuel += computeFuel(it) }
    println("Solution for part 1: $fuel")
}

fun main() {
    val numbers = readFile("files/day1.txt")
    part1(numbers)
}