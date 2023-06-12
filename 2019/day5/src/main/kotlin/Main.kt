import org.jetbrains.annotations.Nullable
import java.io.File
import java.nio.charset.Charset
import kotlin.math.max

enum class OpCode(val code: Int) {
    ADD(1),
    MULTIPLY(2),
    INPUT(3),
    OUTPUT(4),
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

data class Instruction(val opCode: OpCode, val parameters: List<Parameter>) {
    init {
        when (this.opCode) {
            OpCode.ADD -> assert(this.parameters.size == 3)
            OpCode.MULTIPLY -> assert(this.parameters.size == 3)
            OpCode.INPUT -> assert(this.parameters.size == 1)
            OpCode.OUTPUT -> assert(this.parameters.size == 1)
            OpCode.STOP -> assert(this.parameters.isEmpty())
            else -> throw NotImplementedError("Instruction not implemented for OpCode ${this.opCode}.")
        }
    }

    fun apply(code: MutableList<Int>): Int? {
        when (this.opCode) {
            OpCode.ADD -> this.add(code)
            OpCode.MULTIPLY -> this.multiply(code)
            OpCode.INPUT -> this.input(code)
            OpCode.OUTPUT -> return this.output(code)
            OpCode.STOP -> {}
            else -> throw NotImplementedError("Instruction.apply not implemented for OpCode ${this.opCode}.")
        }
        return null
    }

    private fun add(code: MutableList<Int>) {
        code[this.parameters[2].value] = this.parameters[0].getValue(code) + this.parameters[1].getValue(code)
    }

    private fun multiply(code: MutableList<Int>) {
        code[this.parameters[2].value] = this.parameters[0].getValue(code) * this.parameters[1].getValue(code)
    }

    private fun input(code: MutableList<Int>) {
        print("Provide integer parameter input: ")
        val userInput = readln().toInt()
        code[this.parameters[0].getValue(code)] = userInput
    }

    private fun output(code: MutableList<Int>): Int {
        val out = this.parameters[0].getValue(code)
        println("Intermediate Output Value: ${out}.")
        return out
    }
}

class ParameterParser() {
    fun parse(code: List<Int>, instructionPointer: Int, opCode: OpCode): List<Parameter> {
        return when (opCode) {
            OpCode.ADD, OpCode.MULTIPLY -> this.parseAddOrMultiplyParameters(code, instructionPointer)
            OpCode.INPUT -> this.parseInputParameter(code, instructionPointer)
            OpCode.OUTPUT -> this.parseOutputParameter(code, instructionPointer)
            OpCode.STOP -> listOf()
            else -> throw NotImplementedError("Parameter parser does not implement OpCode ${opCode}.")
        }
    }

    private fun parseAddOrMultiplyParameters(code: List<Int>, instructionPointer: Int): List<Parameter> {
        var opCodeString = code[instructionPointer].toString()
        opCodeString = opCodeString.padStart(5, '0')
        assert(opCodeString[0] == '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[2]), code[instructionPointer + 1]),
            Parameter(ParameterMode.fromChar(opCodeString[1]), code[instructionPointer + 2]),
            Parameter(ParameterMode.fromChar(opCodeString[0]), code[instructionPointer + 3]),
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
}

class ShipComputer(inputCode: List<Int>) {
    private val code = inputCode.toMutableList()
    private var instructionPointer = 0
    private val parameterParser = ParameterParser()

    fun parse(): Int {
        var output = 0
        while (true) {
            val instruction = parseInstructionCode()
            val out = instruction.apply(this.code)
            when (instruction.opCode) {
                OpCode.OUTPUT -> output = out ?: throw Error("Output instruction should return a valid Integer value.")
                OpCode.STOP -> break
                else -> {}
            }
            this.increaseInstructionPointer(instruction.opCode)
        }
        return output
    }

    private fun parseInstructionCode(): Instruction {
        val opCode = this.getOpCode()
        val parameters = this.parameterParser.parse(this.code, this.instructionPointer, opCode)
        return Instruction(opCode, parameters)
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
            OpCode.STOP -> 0
            else -> throw NotImplementedError("increaseInstructionPointer is not implemented for OpCode ${opCode}.")
        }
    }
}

fun readFile(file: String): List<Int> {
    return File(file).readText(Charset.defaultCharset()).split(",").map { it.toInt() }
}

fun part1(code: List<Int>) {
    val computer = ShipComputer(code)
    val result = computer.parse()
    println("Result of part 1: ${result}.")
}

fun main() {
    val code = readFile("files/day5.txt")
    part1(code)
}
