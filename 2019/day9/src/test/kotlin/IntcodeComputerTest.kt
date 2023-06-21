import kotlin.random.Random
import kotlin.test.Test
import kotlin.test.assertEquals

class IntcodeComputerTest {

    private fun stringToLongList(string: String): List<Long> {
        return string.split(",").map { it.toLong() }
    }

    private data class TestInput(
        val code: List<Long>,
        val expectedOutput: Long
    )

    @Test
    fun testProcess() {
        val tests = listOf(
            TestInput(this.stringToLongList("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"), 109),
            TestInput(this.stringToLongList("1102,34915192,34915192,7,4,7,99,0"), 1219070632396864),
            TestInput(this.stringToLongList("104,1125899906842624,99"), 1125899906842624),
            TestInput(this.stringToLongList("109,-1,4,1,99"), -1),
            TestInput(this.stringToLongList("109,-1,104,1,99"), 1),
            TestInput(this.stringToLongList("109,-1,204,1,99"), 109),
            TestInput(this.stringToLongList("109,1,9,2,204,-6,99"), 204),
            TestInput(this.stringToLongList("109,1,109,9,204,-6,99"), 204),
            TestInput(this.stringToLongList("109,1,209,-1,204,-106,99"), 204),
        )
        for (test in tests) {
            val (code, expectedOutput) = test
            val computer = IntcodeComputer(code)
            assertEquals(expectedOutput, computer.process())
        }
    }

    @Test
    fun testProcessWithInput() {
        val input = Random.nextLong()
        val tests = listOf(
            TestInput(this.stringToLongList("109,1,3,3,204,2,99"), input),
            TestInput(this.stringToLongList("109,1,203,2,204,2,99"), input),
        )
        for (test in tests) {
            val (code, expectedOutput) = test
            val computer = IntcodeComputer(code)
            computer.pushInput(input)
            assertEquals(expectedOutput, computer.process())
        }
    }

    @Test
    fun testPart1() {
        val intcode = readFile("./files/day9.txt")
        val shipComputer = IntcodeComputer(intcode)
        shipComputer.pushInput(1)
        val result = shipComputer.process()
        assertEquals(3507134798, result)
    }

    @Test
    fun testPart2() {
        val intcode = readFile("./files/day9.txt")
        val shipComputer = IntcodeComputer(intcode)
        shipComputer.pushInput(2)
        val result = shipComputer.process()
        assertEquals(84513, result)
    }
}