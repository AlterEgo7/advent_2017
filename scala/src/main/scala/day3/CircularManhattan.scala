package day3

import math._

object CircularManhattan {

  def main(args: Array[String]): Unit = {
    val input = 347991

    println(manhattan(input))
  }

  def manhattan(index: Int): Int = {
    if (index == 1) return 0

    val size = ceil(sqrt(index)).toInt / 2

    val capacities = 1 +: (2 to size + 1).map{ i =>
      val side = 2 * i - 1
      side * 2 + (side - 2) * 2
    }

    var cum_sum = 0

    val indexes = capacities.map { i =>
      cum_sum += i
      cum_sum
    }

    val rev_indexes = indexes.reverse

    val offset = index - rev_indexes(1)

    def getCircular(offset: Int, size: Int) = {
      val modOffset = abs((offset - size) % (size * 2))

      min(abs(size * 2 - modOffset), abs(modOffset))
    }

    getCircular(offset, size) + size
  }
}
