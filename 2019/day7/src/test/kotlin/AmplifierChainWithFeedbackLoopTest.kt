import kotlin.math.max
import kotlin.test.Test
import kotlin.test.assertEquals

class AmplifierChainWithFeedbackLoopTest {
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
                stringToIntList("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
                0,
                listOf(9, 8, 7, 6, 5),
                139629729
            ),
            TestInput(
                stringToIntList("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
                0,
                listOf(9,7,8,5,6),
                18216
            ),
        )
        for (test in tests) {
            val (code, input, phaseSettings, expectedOutput) = test
            val chain = AmplifierChainWithFeedbackLoop(code, phaseSettings, 100)
            assertEquals(expectedOutput, chain.getOutputSignal(input))
        }
    }

    @Test
    fun testPart2() {
        val code = readFile("files/day7.txt")
        val maxSignal = part2(code)
        assertEquals(61379886, maxSignal)
    }

}