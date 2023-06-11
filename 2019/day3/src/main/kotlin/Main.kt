import java.io.File
import java.security.InvalidParameterException
import javax.management.openmbean.InvalidKeyException
import kotlin.math.absoluteValue

enum class Direction(val direction: Int) {
    Up(1),
    Right(2),
    Down(3),
    Left(4),
}

data class Movement(val direction: Direction, val range: Int) {
    fun getDirectionVector(): Pair<Int, Int> {
        return when (direction) {
            Direction.Up -> Pair(0, 1)
            Direction.Right -> Pair(1, 0)
            Direction.Down -> Pair(0, -1)
            Direction.Left -> Pair(-1, 0)
        }
    }
}

fun instructionToMovement(string: String): Movement {
    val direction = when (string[0]) {
        'U' -> Direction.Up
        'R' -> Direction.Right
        'D' -> Direction.Down
        'L' -> Direction.Left
        else -> throw IllegalArgumentException("Unknown direction ${string[0]}.")
    }
    val range = string.substring(1).toInt()
    return Movement(direction, range)
}

fun stringToMovements(string: String): List<Movement> {
    return string.split(",").map { instructionToMovement(it) }
}

fun readFile(file: String): Pair<List<Movement>, List<Movement>> {
    val mLists = mutableListOf<List<Movement>>()
    for (line in File(file).readLines()) {
        mLists.add(stringToMovements(line))
    }
    assert(mLists.size == 2)
    return Pair(mLists[0], mLists[1])
}

fun getVisitedPoints(path: List<Movement>): Pair<Set<Pair<Int, Int>>, Map<Pair<Int, Int>, Int>> {
    var currentPoint = Pair(0, 0)
    val visitedPoints = mutableSetOf<Pair<Int, Int>>()
    val pointToSteps = mutableMapOf<Pair<Int, Int>, Int>()
    var steps = 0

    for (movement in path) {
        val direction = movement.getDirectionVector()
        for (i in 0 until movement.range) {
            currentPoint = Pair(currentPoint.first + direction.first, currentPoint.second + direction.second)
            steps += 1
            if (currentPoint !in pointToSteps.keys) {
                pointToSteps[currentPoint] = steps
            }
            visitedPoints.add(currentPoint)
        }
    }

    return Pair(visitedPoints, pointToSteps)
}

fun getIntersections(path1: List<Movement>, path2: List<Movement>): List<Triple<Int, Int, Int>> {
    val intersections = mutableListOf<Triple<Int, Int, Int>>()

    val path1Points = getVisitedPoints(path1)
    val path2Points = getVisitedPoints(path2)

    for (point in path1Points.first) {
        if (point != Pair(0, 0) && point in path2Points.first) {
            intersections.add(
                Triple(
                    point.first,
                    point.second,
                    (path1Points.second[point] ?: throw InvalidKeyException()) + (path2Points.second[point]
                        ?: throw InvalidKeyException())
                )
            )
        }
    }

    return intersections
}

fun getClosestManhattanDistance(intersections: List<Triple<Int, Int, Int>>): Int {
    return intersections.minOfOrNull { it.first.absoluteValue + it.second.absoluteValue }
        ?: throw InvalidParameterException("Empty list")
}

fun getMinimalManhattanDistanceOfIntersections(mLists: Pair<List<Movement>, List<Movement>>): Int {
    val intersections = getIntersections(mLists.first, mLists.second)
    return getClosestManhattanDistance(intersections)
}
fun getMinimalSignalDelay(mLists: Pair<List<Movement>, List<Movement>>): Int {
    val intersections = getIntersections(mLists.first, mLists.second)
    return intersections.minOfOrNull { it.third }
        ?: throw InvalidParameterException("Empty list")
}

fun part1(mLists: Pair<List<Movement>, List<Movement>>) {
    val minDistance = getMinimalManhattanDistanceOfIntersections(mLists)
    println("The Manhattan distance to the closest intersection is ${minDistance}.")
}

fun part2(mLists: Pair<List<Movement>, List<Movement>>) {
    val minDistance = getMinimalSignalDelay(mLists)
    println("The signal delay to the closest intersection is ${minDistance}.")
}

fun main() {
    val mLists = readFile("files/day3.txt")
    part1(mLists)
    part2(mLists)
}