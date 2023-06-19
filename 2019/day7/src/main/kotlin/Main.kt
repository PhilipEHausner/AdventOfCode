import java.io.File
import java.nio.charset.Charset
import kotlin.math.max

class Amplifier(code: List<Int>, phaseSetting: Int) {
    private val computer = IntcodeComputer(code)

    init {
        computer.pushInput(phaseSetting)
    }

    fun process(): Int {
        return computer.process()
    }

    fun pushInput(input: Int) {
        this.computer.pushInput(input)
    }
}

class AmplifierChain(private val code: List<Int>, private val phaseSettings: List<Int>) {
    fun getOutputSignal(input: Int): Int {
        var currInput = input
        for (setting in this.phaseSettings) {
            val amplifier = Amplifier(this.code, setting)
            amplifier.pushInput(currInput)
            currInput = amplifier.process()
        }
        return currInput
    }
}

class AmplifierChainWithFeedbackLoop(
    private val code: List<Int>,
    private val phaseSettings: List<Int>,
    private val rounds: Int
) {
    fun getOutputSignal(input: Int): Int {
        val amplifiers = mutableListOf<Amplifier>()
        for (setting in this.phaseSettings) {
            amplifiers.add(Amplifier(this.code, setting))
        }

        var currInput = input
        var maxOutput = Int.MIN_VALUE
        for (i in 1..this.rounds) {
            for (amplifier in amplifiers) {
                amplifier.pushInput(currInput)
                currInput = amplifier.process()
                maxOutput = max(maxOutput, currInput)
            }
        }

        return maxOutput
    }
}

fun readFile(file: String): List<Int> {
    return File(file).readText(Charset.defaultCharset()).split(",").map { it.toInt() }
}

fun <T> getPermutations(list: List<T>): List<List<T>> {
    if (list.isEmpty()) return emptyList()

    fun <T> allPermutations(list: List<T>): Set<List<T>> {
        if (list.isEmpty()) return setOf(emptyList())

        val result: MutableSet<List<T>> = mutableSetOf()
        for (i in list.indices) {
            allPermutations(list - list[i]).forEach { item ->
                result.add(item + list[i])
            }
        }
        return result
    }

    return allPermutations(list).toList()
}

fun part1(code: List<Int>): Int {
    val phaseSettings = getPermutations(listOf(0, 1, 2, 3, 4))
    var maxSignal = Int.MIN_VALUE
    for (phaseSetting in phaseSettings) {
        val chain = AmplifierChain(code, phaseSetting)
        val signal = chain.getOutputSignal(0)
        maxSignal = max(maxSignal, signal)
    }
    println("Result of part 1: ${maxSignal}.")
    return maxSignal
}

fun part2(code: List<Int>): Int {
    val phaseSettings = getPermutations(listOf(5, 6, 7, 8, 9))
    var maxSignal = Int.MIN_VALUE
    for (phaseSetting in phaseSettings) {
        val chain = AmplifierChainWithFeedbackLoop(code, phaseSetting, 100)
        val signal = chain.getOutputSignal(0)
        maxSignal = max(maxSignal, signal)
    }
    println("Result of part 2: ${maxSignal}.")
    return maxSignal
}

fun main() {
    val code = readFile("files/day7.txt")
    part1(code)
    part2(code)
}