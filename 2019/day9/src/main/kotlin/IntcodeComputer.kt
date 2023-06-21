import kotlin.math.max

class IntcodeComputer(intcode: List<Long>) {
    private val code: MutableMap<Long, Long>
    private val inputs = mutableListOf<Long>()
    private var instructionPointer: Long = 0
    private val parameterParser = ParameterParser()
    private var relativeBase: Long = 0

    init {
        code =
            intcode.mapIndexed { i: Int, number: Long -> i.toLong() to number }.toMap().toMutableMap().withDefault { 0 }
    }

    fun process(): Long {
        while (true) {
            when (getOpCode()) {
                OpCode.ADD -> this.processAdd()
                OpCode.MULTIPLY -> this.processMultiply()
                OpCode.INPUT -> this.processInput()
                OpCode.OUTPUT -> return this.processOutput()
                OpCode.JUMP_IF_TRUE -> this.processJumpIfTrue()
                OpCode.JUMP_IF_FALSE -> this.processJumpIfFalse()
                OpCode.LESS_THAN -> this.processLessThan()
                OpCode.EQUALS -> this.processEquals()
                OpCode.ADJUST_RELATIVE_BASE -> this.processAdjustRelativeBase()
                OpCode.STOP -> break
            }

        }
        return 0
    }

    fun pushInput(input: Long) {
        this.inputs.add(input)
    }

    private fun getOpCode(): OpCode {
        val cmd = this.code[instructionPointer].toString()
        val opCode = cmd.substring(max(0, cmd.length - 2)).toLong()
        return OpCode.fromLong(opCode)
    }

    private fun processAdd() {
        val parameters = this.parameterParser.parseAddOrMultiplyParameters(this.code, this.instructionPointer)
        val codeState = this.getCodeStateForParameter()
        this.code[parameters[2].getValue(codeState)] =
            parameters[0].getValue(codeState) + parameters[1].getValue(codeState)
        this.instructionPointer += 4
    }

    private fun processMultiply() {
        val parameters = this.parameterParser.parseAddOrMultiplyParameters(this.code, this.instructionPointer)
        val codeState = this.getCodeStateForParameter()
        this.code[parameters[2].getValue(codeState)] =
            parameters[0].getValue(codeState) * parameters[1].getValue(codeState)
        this.instructionPointer += 4
    }

    private fun processInput() {
        assert(this.inputs.size > 0) { "Input array is empty." }
        val parameters = this.parameterParser.parseInputParameter(this.code, this.instructionPointer)
        val codeState = this.getCodeStateForParameter()
        val input = this.inputs.removeFirst()
        code[parameters[0].getValue(codeState)] = input
        this.instructionPointer += 2
    }

    private fun processOutput(): Long {
        val parameters = this.parameterParser.parseOutputParameter(this.code, this.instructionPointer)
        val codeState = this.getCodeStateForParameter()
        this.instructionPointer += 2
        return parameters[0].getValue(codeState)
    }

    private fun processJumpIfTrue() {
        val parameters = this.parameterParser.parseJumpIfParameter(this.code, this.instructionPointer)
        val codeState = this.getCodeStateForParameter()
        when (parameters[0].getValue(codeState)) {
            0.toLong() -> this.instructionPointer += 3
            else -> this.instructionPointer = parameters[1].getValue(codeState)
        }
    }

    private fun processJumpIfFalse() {
        val parameters = this.parameterParser.parseJumpIfParameter(this.code, this.instructionPointer)
        val codeState = this.getCodeStateForParameter()
        when (parameters[0].getValue(codeState)) {
            0.toLong() -> this.instructionPointer = parameters[1].getValue(codeState)
            else -> this.instructionPointer += 3
        }
    }

    private fun processLessThan() {
        val parameters = this.parameterParser.parseComparisonParameter(this.code, this.instructionPointer)
        val codeState = this.getCodeStateForParameter()
        this.code[parameters[2].getValue(codeState)] =
            when (parameters[0].getValue(codeState) < parameters[1].getValue(codeState)) {
                true -> 1
                false -> 0
            }
        this.instructionPointer += 4
    }

    private fun processEquals() {
        val parameters = this.parameterParser.parseComparisonParameter(this.code, this.instructionPointer)
        val codeState = this.getCodeStateForParameter()
        this.code[parameters[2].getValue(codeState)] =
            when (parameters[0].getValue(codeState) == parameters[1].getValue(codeState)) {
                true -> 1
                false -> 0
            }
        this.instructionPointer += 4
    }

    private fun processAdjustRelativeBase() {
        val parameters = this.parameterParser.parseAdjustRelativeBaseParameter(this.code, this.instructionPointer)
        val codeState = this.getCodeStateForParameter()
        this.relativeBase += parameters[0].getValue(codeState)
        this.instructionPointer += 2
    }

    private fun getCodeStateForParameter(): CodeStateForParameter {
        return CodeStateForParameter(this.code, this.relativeBase)
    }
}

enum class OpCode(val code: Long) {
    ADD(1),
    MULTIPLY(2),
    INPUT(3),
    OUTPUT(4),
    JUMP_IF_TRUE(5),
    JUMP_IF_FALSE(6),
    LESS_THAN(7),
    EQUALS(8),
    ADJUST_RELATIVE_BASE(9),
    STOP(99);

    companion object {
        fun fromLong(value: Long) = OpCode.values().first { it.code == value }
    }
}

enum class ParameterMode(val mode: Long) {
    PositionReadMode(0),
    ImmediateReadMode(1),
    RelativeReadMode(2),
    PositionWriteMode(10),
    ImmediateWriteMode(11),
    RelativeWriteMode(12);

    companion object {
        fun fromChar(value: Char, interactionMode: InteractionMode) = ParameterMode.values().first {
            when (interactionMode) {
                InteractionMode.Read -> it.mode == value.toString().toLong()
                InteractionMode.Write -> it.mode == value.toString().toLong() + 10
            }
        }
    }
}

enum class InteractionMode(val mode: Int) {
    Read(0),
    Write(1),
}

data class CodeStateForParameter(val code: Map<Long, Long>, val relativeBase: Long)

data class Parameter(val mode: ParameterMode, val value: Long) {
    fun getValue(codeState: CodeStateForParameter): Long {
        return when (this.mode) {
            ParameterMode.PositionReadMode -> codeState.code.getValue(value)
            ParameterMode.ImmediateReadMode -> value
            ParameterMode.RelativeReadMode -> codeState.code.getValue(value + codeState.relativeBase)
            ParameterMode.PositionWriteMode -> value
            ParameterMode.ImmediateWriteMode -> throw Error("Instructions will never write in Immediate Mode.")
            ParameterMode.RelativeWriteMode -> value + codeState.relativeBase
        }
    }
}

class ParameterParser() {
    fun parseAddOrMultiplyParameters(code: Map<Long, Long>, instructionPointer: Long): List<Parameter> {
        var opCodeString = this.getOpCodeString(code, instructionPointer)
        opCodeString = opCodeString.padStart(5, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[2], InteractionMode.Read), code.getValue(instructionPointer + 1)),
            Parameter(ParameterMode.fromChar(opCodeString[1], InteractionMode.Read), code.getValue(instructionPointer + 2)),
            Parameter(ParameterMode.fromChar(opCodeString[0], InteractionMode.Write), code.getValue(instructionPointer + 3)),
        )
    }

    fun parseInputParameter(code: Map<Long, Long>, instructionPointer: Long): List<Parameter> {
        var opCodeString = this.getOpCodeString(code, instructionPointer)
        opCodeString = opCodeString.padStart(3, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[0], InteractionMode.Write), code.getValue(instructionPointer + 1))
        )
    }

    fun parseOutputParameter(code: Map<Long, Long>, instructionPointer: Long): List<Parameter> {
        var opCodeString = this.getOpCodeString(code, instructionPointer)
        opCodeString = opCodeString.padStart(3, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[0], InteractionMode.Read), code.getValue(instructionPointer + 1))
        )
    }

    fun parseJumpIfParameter(code: Map<Long, Long>, instructionPointer: Long): List<Parameter> {
        var opCodeString = this.getOpCodeString(code, instructionPointer)
        opCodeString = opCodeString.padStart(4, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[1], InteractionMode.Read), code.getValue(instructionPointer + 1)),
            Parameter(ParameterMode.fromChar(opCodeString[0], InteractionMode.Read), code.getValue(instructionPointer + 2)),
        )
    }

    fun parseComparisonParameter(code: Map<Long, Long>, instructionPointer: Long): List<Parameter> {
        var opCodeString = this.getOpCodeString(code, instructionPointer)
        opCodeString = opCodeString.padStart(5, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[2], InteractionMode.Read), code.getValue(instructionPointer + 1)),
            Parameter(ParameterMode.fromChar(opCodeString[1], InteractionMode.Read), code.getValue(instructionPointer + 2)),
            Parameter(ParameterMode.fromChar(opCodeString[0], InteractionMode.Write), code.getValue(instructionPointer + 3)),
        )
    }

    fun parseAdjustRelativeBaseParameter(code: Map<Long, Long>, instructionPointer: Long): List<Parameter> {
        var opCodeString = this.getOpCodeString(code, instructionPointer)
        opCodeString = opCodeString.padStart(3, '0')
        return listOf(
            Parameter(ParameterMode.fromChar(opCodeString[0], InteractionMode.Read), code.getValue(instructionPointer + 1))
        )
    }

    private fun getOpCodeString(code: Map<Long, Long>, instructionPointer: Long): String {
        return code[instructionPointer].toString()
    }
}
