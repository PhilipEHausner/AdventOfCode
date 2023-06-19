import kotlin.math.max

class IntcodeComputer(intcode: List<Int>) {
    private val code = intcode.toMutableList()
    private val inputs = mutableListOf<Int>()
    private var instructionPointer = 0
    private val parameterParser = ParameterParser()

    fun process(): Int {
        while (true) {
            when (getOpCode()) {
                OpCode.ADD -> this.processAdd()
                OpCode.MULTIPLY -> this.processMultiply()
                OpCode.INPUT -> this.processInput()
                OpCode.OUTPUT -> return this.processOutput()
                OpCode.JUMP_IF_TRUE -> this.processParseIfTrue()
                OpCode.JUMP_IF_FALSE -> this.processParseIfFalse()
                OpCode.LESS_THAN -> this.processLessThan()
                OpCode.EQUALS -> this.processEquals()
                OpCode.STOP -> break
            }

        }
        return 0
    }

    fun pushInput(input: Int) {
        this.inputs.add(input)
    }

    private fun getOpCode(): OpCode {
        val cmd = this.code[instructionPointer].toString()
        val opCode = cmd.substring(max(0, cmd.length - 2)).toInt()
        return OpCode.fromInt(opCode)
    }

    private fun processAdd() {
        val parameters = this.parameterParser.parseAddOrMultiplyParameters(this.code, this.instructionPointer)
        this.code[parameters[2].getValue(this.code)] =
            parameters[0].getValue(this.code) + parameters[1].getValue(this.code)
        this.instructionPointer += 4
    }

    private fun processMultiply() {
        val parameters = this.parameterParser.parseAddOrMultiplyParameters(this.code, this.instructionPointer)
        this.code[parameters[2].getValue(this.code)] =
            parameters[0].getValue(this.code) * parameters[1].getValue(this.code)
        this.instructionPointer += 4
    }

    private fun processInput() {
        assert(this.inputs.size > 0) { "Input array is empty." }
        val parameters = this.parameterParser.parseInputParameter(this.code, this.instructionPointer)
        val input = this.inputs.removeFirst()
        code[parameters[0].getValue(code)] = input
        this.instructionPointer += 2
    }

    private fun processOutput(): Int {
        val parameters = this.parameterParser.parseOutputParameter(this.code, this.instructionPointer)
        this.instructionPointer += 2
        return parameters[0].getValue(code)
    }

    private fun processParseIfTrue() {
        val parameters = this.parameterParser.parseJumpIfParameter(this.code, this.instructionPointer)
        when (parameters[0].getValue(code)) {
            0 -> this.instructionPointer += 3
            else -> this.instructionPointer = parameters[1].getValue(code)
        }
    }

    private fun processParseIfFalse() {
        val parameters = this.parameterParser.parseJumpIfParameter(this.code, this.instructionPointer)
        when (parameters[0].getValue(code)) {
            0 -> this.instructionPointer = parameters[1].getValue(code)
            else -> this.instructionPointer += 3
        }
    }

    private fun processLessThan() {
        val parameters = this.parameterParser.parseComparisonParameter(this.code, this.instructionPointer)
        this.code[parameters[2].getValue(code)] =
            when (parameters[0].getValue(code) < parameters[1].getValue(code)) {
                true -> 1
                false -> 0
            }
        this.instructionPointer += 4
    }

    private fun processEquals() {
        val parameters = this.parameterParser.parseComparisonParameter(this.code, this.instructionPointer)
        this.code[parameters[2].getValue(code)] =
            when (parameters[0].getValue(code) == parameters[1].getValue(code)) {
                true -> 1
                false -> 0
            }
        this.instructionPointer += 4
    }
}

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

class ParameterParser() {
    fun parseAddOrMultiplyParameters(code: List<Int>, instructionPointer: Int): List<Parameter> {
        var opCodeString = code[instructionPointer].toString()
        opCodeString = opCodeString.padStart(5, '0')
        assert(opCodeString[0] == '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[2]), code[instructionPointer + 1]),
            Parameter(ParameterMode.fromChar(opCodeString[1]), code[instructionPointer + 2]),
            Parameter(ParameterMode.ImmediateMode, code[instructionPointer + 3]),
        )
    }

    fun parseInputParameter(code: List<Int>, instructionPointer: Int): List<Parameter> {
        return listOf(
            Parameter(ParameterMode.ImmediateMode, code[instructionPointer + 1])
        )
    }

    fun parseOutputParameter(code: List<Int>, instructionPointer: Int): List<Parameter> {
        var opCodeString = code[instructionPointer].toString()
        opCodeString = opCodeString.padStart(3, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[0]), code[instructionPointer + 1])
        )
    }

    fun parseJumpIfParameter(code: List<Int>, instructionPointer: Int): List<Parameter> {
        var opCodeString = code[instructionPointer].toString()
        opCodeString = opCodeString.padStart(4, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[1]), code[instructionPointer + 1]),
            Parameter(ParameterMode.fromChar(opCodeString[0]), code[instructionPointer + 2]),
        )
    }

    fun parseComparisonParameter(code: List<Int>, instructionPointer: Int): List<Parameter> {
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
