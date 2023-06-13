import java.io.File
import java.nio.charset.Charset
import kotlin.math.max

enum class OpCode(val code: Int) {
    ADD(1),
    MULTIPLY(2),
    INPUT(3),
    OUTPUT(4),
    JUMP_IF_TRUE(5),
    JUMP_IF_FALSE(6),
    LESS_THAN(7),
    EQUALS(8),
    STOP(99);

    companion object {
        fun fromInt(value: Int) = OpCode.values().first { it.code == value }
    }
}

enum class ParameterMode(val mode: Int) {
    PositionMode(0),
    ImmediateMode(1);

    companion object {
        fun fromChar(value: Char) = ParameterMode.values().first { it.mode == value.toString().toInt() }
    }
}

data class Parameter(val mode: ParameterMode, val value: Int) {
    fun getValue(code: List<Int>): Int {
        return when (this.mode) {
            ParameterMode.PositionMode -> code[value]
            ParameterMode.ImmediateMode -> value
        }
    }
}

data class Instruction(
    val opCode: OpCode,
    val parameters: List<Parameter>,
    private val defaultInputs: MutableList<Int>? = null
) {
    init {
        when (this.opCode) {
            OpCode.ADD -> assert(this.parameters.size == 3)
            OpCode.MULTIPLY -> assert(this.parameters.size == 3)
            OpCode.INPUT -> assert(this.parameters.size == 1)
            OpCode.OUTPUT -> assert(this.parameters.size == 1)
            OpCode.JUMP_IF_TRUE -> assert(this.parameters.size == 2)
            OpCode.JUMP_IF_FALSE -> assert(this.parameters.size == 2)
            OpCode.LESS_THAN -> assert(this.parameters.size == 3)
            OpCode.EQUALS -> assert(this.parameters.size == 3)
            OpCode.STOP -> assert(this.parameters.isEmpty())
        }
    }

    fun apply(code: MutableList<Int>): Int? {
        var result: Int? = null
        when (this.opCode) {
            OpCode.ADD -> this.add(code)
            OpCode.MULTIPLY -> this.multiply(code)
            OpCode.INPUT -> this.input(code)
            OpCode.OUTPUT -> result = this.output(code)
            OpCode.JUMP_IF_TRUE -> result = this.jumpIfTrue(code)
            OpCode.JUMP_IF_FALSE -> result = this.jumpIfFalse(code)
            OpCode.LESS_THAN -> this.lessThan(code)
            OpCode.EQUALS -> this.equality(code)
            OpCode.STOP -> {}
        }
        return result
    }

    private fun add(code: MutableList<Int>) {
        code[this.parameters[2].getValue(code)] = this.parameters[0].getValue(code) + this.parameters[1].getValue(code)
    }

    private fun multiply(code: MutableList<Int>) {
        code[this.parameters[2].getValue(code)] = this.parameters[0].getValue(code) * this.parameters[1].getValue(code)
    }

    private fun input(code: MutableList<Int>) {
        val userInput = if (this.defaultInputs != null) {
//            if (this.defaultInputs.size > 1) {
            this.defaultInputs.removeFirst()
//            } else {
//                this.defaultInputs[0]
//            }
        } else {
            print("Provide integer parameter input: ")
            readln().toInt()
        }
        code[this.parameters[0].getValue(code)] = userInput
    }

    private fun output(code: MutableList<Int>): Int {
        //        println("Intermediate Output Value: ${out}.")
        return parameters[0].getValue(code)
    }

    private fun jumpIfTrue(code: List<Int>): Int? {
        return when (this.parameters[0].getValue(code)) {
            0 -> null
            else -> this.parameters[1].getValue(code)
        }
    }

    private fun jumpIfFalse(code: List<Int>): Int? {
        return when (this.parameters[0].getValue(code)) {
            0 -> this.parameters[1].getValue(code)
            else -> null
        }
    }

    private fun lessThan(code: MutableList<Int>) {
        code[this.parameters[2].getValue(code)] =
            when (this.parameters[0].getValue(code) < this.parameters[1].getValue(code)) {
                true -> 1
                false -> 0
            }
    }

    private fun equality(code: MutableList<Int>) {
        code[this.parameters[2].getValue(code)] =
            when (this.parameters[0].getValue(code) == this.parameters[1].getValue(code)) {
                true -> 1
                false -> 0
            }
    }
}

class ParameterParser() {
    fun parse(code: List<Int>, instructionPointer: Int, opCode: OpCode): List<Parameter> {
        return when (opCode) {
            OpCode.ADD, OpCode.MULTIPLY -> this.parseAddOrMultiplyParameters(code, instructionPointer)
            OpCode.INPUT -> this.parseInputParameter(code, instructionPointer)
            OpCode.OUTPUT -> this.parseOutputParameter(code, instructionPointer)
            OpCode.JUMP_IF_TRUE, OpCode.JUMP_IF_FALSE -> this.parseJumpIfParameter(code, instructionPointer)
            OpCode.LESS_THAN, OpCode.EQUALS -> this.parseComparisonParameter(code, instructionPointer)
            OpCode.STOP -> listOf()
        }
    }

    private fun parseAddOrMultiplyParameters(code: List<Int>, instructionPointer: Int): List<Parameter> {
        var opCodeString = code[instructionPointer].toString()
        opCodeString = opCodeString.padStart(5, '0')
        assert(opCodeString[0] == '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[2]), code[instructionPointer + 1]),
            Parameter(ParameterMode.fromChar(opCodeString[1]), code[instructionPointer + 2]),
            Parameter(ParameterMode.ImmediateMode, code[instructionPointer + 3]),
        )
    }

    private fun parseInputParameter(code: List<Int>, instructionPointer: Int): List<Parameter> {
        return listOf(
            Parameter(ParameterMode.ImmediateMode, code[instructionPointer + 1])
        )
    }

    private fun parseOutputParameter(code: List<Int>, instructionPointer: Int): List<Parameter> {

        var opCodeString = code[instructionPointer].toString()
        opCodeString = opCodeString.padStart(3, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[0]), code[instructionPointer + 1])
        )
    }

    private fun parseJumpIfParameter(code: List<Int>, instructionPointer: Int): List<Parameter> {
        var opCodeString = code[instructionPointer].toString()
        opCodeString = opCodeString.padStart(4, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[1]), code[instructionPointer + 1]),
            Parameter(ParameterMode.fromChar(opCodeString[0]), code[instructionPointer + 2]),
        )
    }

    private fun parseComparisonParameter(code: List<Int>, instructionPointer: Int): List<Parameter> {
        var opCodeString = code[instructionPointer].toString()
        opCodeString = opCodeString.padStart(5, '0')
        assert(opCodeString[0] == '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[2]), code[instructionPointer + 1]),
            Parameter(ParameterMode.fromChar(opCodeString[1]), code[instructionPointer + 2]),
            Parameter(ParameterMode.ImmediateMode, code[instructionPointer + 3]),
        )
    }
}

class ShipComputer(inputCode: List<Int>, inputs: List<Int>? = null) {
    private val code = inputCode.toMutableList()
    private var instructionPointer = 0
    private val parameterParser = ParameterParser()
    private var inputs = inputs?.toMutableList()

    fun parse(): Int {
        while (true) {
            val instruction = parseInstructionCode()
            val out = instruction.apply(this.code)
            when (instruction.opCode) {
                OpCode.JUMP_IF_FALSE, OpCode.JUMP_IF_TRUE -> this.instructionPointer =
                    out ?: (this.instructionPointer + 3)

                OpCode.STOP -> break
                else -> {}
            }
            this.increaseInstructionPointer(instruction.opCode)

            if (instruction.opCode == OpCode.OUTPUT) {
                return out
                    ?: throw Error("Output instruction should return a valid Integer value for operation 'OUTPUT'.")
            }
        }
        return 0
    }

    private fun parseInstructionCode(): Instruction {
        val opCode = this.getOpCode()
        val parameters = this.parameterParser.parse(this.code, this.instructionPointer, opCode)
        return Instruction(opCode, parameters, this.inputs)
    }

    private fun getOpCode(): OpCode {
        val cmd = this.code[instructionPointer].toString()
        val opCode = cmd.substring(max(0, cmd.length - 2)).toInt()
        return OpCode.fromInt(opCode)
    }

    private fun increaseInstructionPointer(opCode: OpCode) {
        this.instructionPointer += when (opCode) {
            OpCode.ADD -> 4
            OpCode.MULTIPLY -> 4
            OpCode.INPUT -> 2
            OpCode.OUTPUT -> 2
            OpCode.JUMP_IF_TRUE -> 0
            OpCode.JUMP_IF_FALSE -> 0
            OpCode.LESS_THAN -> 4
            OpCode.EQUALS -> 4
            OpCode.STOP -> 0
        }
    }

    fun addInput(input: Int) {
        this.inputs!!.add(input)
    }
}

class Amplifier(code: List<Int>, phaseSetting: Int, input: Int? = null) {
    private val computer = ShipComputer(code, listOf(phaseSetting))

    init {
        if (input != null) {
            this.computer.addInput(input)
        }
    }

    fun process(): Int {
        return computer.parse()
    }

    fun addInput(input: Int) {
        this.computer.addInput(input)
    }
}

class AmplifierChain(private val code: List<Int>, private val input: Int, private val phaseSettings: List<Int>) {
    fun getOutputSignal(): Int {
        var currInput = this.input
        for (setting in this.phaseSettings) {
            val amplifier = Amplifier(this.code, setting, currInput)
            currInput = amplifier.process()
        }
        return currInput
    }
}

class AmplifierChainWithFeedbackLoop(
    private val code: List<Int>,
    private val input: Int,
    private val phaseSettings: List<Int>,
    private val rounds: Int
) {
    fun getOutputSignal(): Int {
        val amplifiers = mutableListOf<Amplifier>()
        for (setting in this.phaseSettings) {
            amplifiers.add(Amplifier(this.code, setting))
        }

        var currInput = this.input
        var maxOutput = Int.MIN_VALUE
        for (i in 1..this.rounds) {
            for (amplifier in amplifiers) {
                amplifier.addInput(currInput)
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
        val chain = AmplifierChain(code, 0, phaseSetting)
        val signal = chain.getOutputSignal()
        maxSignal = max(maxSignal, signal)
    }
    println("Result of part 1: ${maxSignal}.")
    return maxSignal
}

fun part2(code: List<Int>): Int {
    val phaseSettings = getPermutations(listOf(5, 6, 7, 8, 9))
    var maxSignal = Int.MIN_VALUE
    for (phaseSetting in phaseSettings) {
        val chain = AmplifierChainWithFeedbackLoop(code, 0, phaseSetting, 100)
        val signal = chain.getOutputSignal()
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