fun isValidPassword(number: Int, largeGroupRule: Boolean = false): Boolean {
    val string = number.toString()
    // It is a six-digit number.
    if (string.length != 6) return false

    var lastDigit = -1
    val adjacentDigitsCount = mutableSetOf<Int>()
    var currentAdjacencyCount = 1

    for (c in string) {
        val digit = c.digitToInt()
        // Count number of adjacent digits
        if (digit == lastDigit) {
            currentAdjacencyCount += 1
        } else {
            adjacentDigitsCount.add(currentAdjacencyCount)
            currentAdjacencyCount = 1
        }
        // Going from left to right, the digits never decrease.
        if (digit < lastDigit) return false
        lastDigit = digit
    }
    adjacentDigitsCount.add(currentAdjacencyCount)

    if (largeGroupRule) {
        return 2 in adjacentDigitsCount
    }
    return adjacentDigitsCount.any { it >= 2 }
}

fun numValidPasswords(minNumber: Int, maxNumber: Int, largeGroupRule: Boolean = false): Int {
    var validPasswords = 0
    for (number in minNumber..maxNumber) {
        if (isValidPassword(number, largeGroupRule)) {
            validPasswords += 1
        }
    }
    return validPasswords
}

fun part1(minNumber: Int, maxNumber: Int) {
    println("Number of valid passwords: ${numValidPasswords(minNumber, maxNumber)}.")
}

fun part2(minNumber: Int, maxNumber: Int) {
    println("Number of valid passwords: ${numValidPasswords(minNumber, maxNumber, true)}.")
}

fun main(args: Array<String>) {
    val minNumber = 172851
    val maxNumber = 675869
    part1(minNumber, maxNumber)
    part2(minNumber, maxNumber)
}