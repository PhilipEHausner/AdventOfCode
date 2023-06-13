import kotlin.math.max
import kotlin.test.Test
import kotlin.test.assertEquals

internal class AmplifierChainTest {
    private fun stringToIntList(string: String): List<Int> {
        return string.split(",").map { it.toInt() }
    }

    private data class TestInput(
        val code: List<Int>,
        val input: Int,
        val phaseSettings: List<Int>,
        val expectedOutput: Int
    )

    @Test
    fun testOutput() {
        val tests = listOf(
            TestInput(
                stringToIntList("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
                0,
                listOf(4, 3, 2, 1, 0),
                43210
            ),
            TestInput(
                stringToIntList("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
                0,
                listOf(0,1,2,3,4),
                54321
            ),
            TestInput(
                stringToIntList("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
                0,
                listOf(1,0,4,3,2),
                65210
            ),
        )
        for (test in tests) {
            val (code, input, phaseSettings, expectedOutput) = test
            val chain = AmplifierChain(code, input, phaseSettings)
            assertEquals(expectedOutput, chain.getOutputSignal())
        }
    }

    @Test
    fun testPart1() {
        val code = readFile("files/day7.txt")
        val phaseSettings = getAllPhaseCombinations()
        var maxSignal = Int.MIN_VALUE
        for (phaseSetting in phaseSettings) {
            val chain = AmplifierChain(code, 0, phaseSetting)
            val signal = chain.getOutputSignal()
            maxSignal = max(maxSignal, signal)
        }
        assertEquals(21000, maxSignal)
    }
}