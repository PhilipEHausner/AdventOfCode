class Image(encoding: List<Int>, imageSize: Pair<Int, Int>) {
    private val layers = mutableListOf<ImageLayer>()
    private val width = imageSize.first
    private val height = imageSize.second

    init {
        val pixelsPerLayer = this.width * this.height
        for (i in encoding.indices step pixelsPerLayer) {
            val slice = encoding.slice(i until i + pixelsPerLayer)
            this.layers.add(ImageLayer(slice, this.width))
        }
    }

    fun checkSum(): Int {
        var minValue = Int.MAX_VALUE
        var index = -1
        for ((i, layer) in this.layers.withIndex()) {
            val zeroes = layer.count(0)
            if (zeroes < minValue) {
                minValue = zeroes
                index = i
            }
        }
        return this.layers[index].count(1) * this.layers[index].count(2)
    }
}

class ImageLayer(encoding: List<Int>, width: Int) {
    private val image: List<List<Int>>

    init {
        val tempImage = mutableListOf<List<Int>>()

        for (i in encoding.indices step width) {
            val row = encoding.slice(i until i + width)
            tempImage.add(row)
        }

        this.image = tempImage
    }

    fun count(value: Int): Int {
        return image.sumOf { it -> it.map { if (it == value) 1 else 0 }.sum() }
    }
}
