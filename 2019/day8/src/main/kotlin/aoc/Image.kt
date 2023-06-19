package aoc

class Image(encoding: List<Int>, imageSize: Pair<Int, Int>) {
    private val layers = mutableListOf<ImageLayer>()
    private val width = imageSize.first
    private val height = imageSize.second

    init {
        val pixelsPerLayer = this.width * this.height
        for (i in encoding.indices step pixelsPerLayer) {
            val slice = encoding.slice(i until i + pixelsPerLayer)
            this.layers.add(ImageLayer(slice, this.width, this.height))
        }
    }

    fun checkSum(): Int {
        var minValue = Int.MAX_VALUE
        var index = -1
        for ((i, layer) in this.layers.withIndex()) {
            val zeroes = layer.count(Color.BLACK)
            if (zeroes < minValue) {
                minValue = zeroes
                index = i
            }
        }
        return this.layers[index].count(Color.WHITE) * this.layers[index].count(Color.TRANSPARENT)
    }

    fun decodeImage(): List<List<Color>> {
        val result = mutableListOf<List<Color>>()

        for (x in 0 until this.height) {
            val row = mutableListOf<Color>()
            for (y in 0 until this.width) {
                row.add(this.getVisiblePixel(x, y))
            }
            result.add(row)
        }

        assert(result.size == this.height)
        assert(result.map { it.size == this.width }.all { it })

        return result
    }

    private fun getVisiblePixel(x: Int, y: Int): Color {
        for (layer in this.layers) {
            return when (layer.getPixel(x, y)){
                Color.BLACK -> Color.BLACK
                Color.WHITE -> Color.WHITE
                Color.TRANSPARENT -> continue
            }
        }
        throw Error("No visible pixel for location x=$x and y=$y.")
    }
}

class ImageLayer(encoding: List<Int>, private val width: Int, private val height: Int) {
    private val image: List<List<Color>>

    init {
        val tempImage = mutableListOf<List<Color>>()

        for (i in encoding.indices step width) {
            val row = encoding.slice(i until i + width).map { Color.fromInt(it) }
            tempImage.add(row)
        }

        this.image = tempImage
    }

    fun count(value: Color): Int {
        return image.sumOf { it -> it.map { if (it == value) 1 else 0 }.sum() }
    }

    fun getPixel(x: Int, y: Int): Color {
        if (x >= this.height || y >= this.width) {
            throw IndexOutOfBoundsException("Pixel location is outside of image.")
        }
        return this.image[x][y]
    }
}

enum class Color(val code: Int) {
    BLACK(0),
    WHITE(1),
    TRANSPARENT(2);

    companion object {
        fun fromInt(value: Int) = Color.values().first { it.code == value }
    }
}
