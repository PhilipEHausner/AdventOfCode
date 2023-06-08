import java.io.File
import java.lang.IllegalArgumentException
import java.nio.charset.Charset

enum class OpCode(val code: Int) {
    ADD(1),
    MULTIPLY(2),
    STOP(99)
}

fun readFile(file: String): List<Int> {
    return File(file).readText(Charset.defaultCharset()).split(",").map { it.toInt() }
}

fun add(numbers: List<Int>, index1: Int, index2: Int): Int {
    return numbers[index1] + numbers[index2]
}

fun multiply(numbers: List<Int>, index1: Int, index2: Int): Int {
    return numbers[index1] * numbers[index2]
}

fun computeProgramAlarm(nums: List<Int>): Int {
    val numbers = nums.toMutableList()
    for (index in numbers.indices step 4) {
        val opCode = numbers[index]
        if (opCode == OpCode.STOP.code) break
        val in1 = numbers[index+1]
        val in2 = numbers[index+2]
        val out = numbers[index+3]
        when (opCode) {
            OpCode.ADD.code -> numbers[out] = add(numbers, in1, in2)
            OpCode.MULTIPLY.code -> numbers[out] = multiply(numbers, in1, in2)
            else -> throw IllegalArgumentException("Invalid opCode: $opCode")
        }
    }
    return numbers[0]
}

fun compute1202ProgramAlarm(nums: List<Int>): Int {
    return computeNounVerbProgramAlarm(nums, 12, 2)
}

fun computeNounVerbProgramAlarm(nums: List<Int>, noun: Int, verb: Int): Int {
    val numbers = nums.toMutableList()
    numbers[1] = noun
    numbers[2] = verb
    return computeProgramAlarm(numbers)
}

fun part1(numbers: List<Int>) {
    val programAlarm = compute1202ProgramAlarm(numbers)
    println("Solution part 1: $programAlarm")
}

fun part2(numbers: List<Int>) {
    var found = false
    for (noun in 0..99) {
        for (verb in 0..99) {
            val alarm = computeNounVerbProgramAlarm(numbers, noun, verb)
            if (alarm == 19690720) {
                println("Solution part 2: 100 * $noun + $verb = ${100 * noun + verb}")
                found = true
                break
            }
        }
        if (found) break
    }
}

fun main() {
    val numbers = readFile("files/day2.txt")
    part1(numbers)
    part2(numbers)
}