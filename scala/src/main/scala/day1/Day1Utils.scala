package day1

object Day1Utils {

  def calculate_sum(data: Array[Int]): Int = {
    data.zipWithIndex
      .filter {
        case (elem, index) =>
          elem == data((index + (data.length / 2)) % data.length)
      }
      .map(_._1)
      .sum
  }

  def parseString(input: String): Array[Int] = {
    input.toCharArray.map(_.asDigit)
  }
}
