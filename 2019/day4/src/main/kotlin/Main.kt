fun isValidPassword(number: Int): Boolean {
    val string = number.toString()
    // It is a six-digit number.
    if (string.length != 6) return false
    var lastDigit = -1
    var twoAdjacentDigitsSame = false
    for (c in string) {
        val digit = c.digitToInt()
        // Two adjacent digits are the same
        if (digit == lastDigit) twoAdjacentDigitsSame = true
        // Going from left to right, the digits never decrease.
        if (digit < lastDigit) return false
        lastDigit = digit
    }
    return twoAdjacentDigitsSame
}

fun numValidPasswords(minNumber: Int, maxNumber: Int): Int {
    var validPasswords = 0
    for (number in minNumber..maxNumber) {
        if (isValidPassword(number)) {
            validPasswords += 1
        }
    }
    return validPasswords
}

fun part1(minNumber: Int, maxNumber: Int) {
    println("Number of valid passwords: ${numValidPasswords(minNumber, maxNumber)}.")
}

fun main(args: Array<String>) {
    val minNumber = 172851
    val maxNumber = 675869
    part1(minNumber, maxNumber)
}