package day1

import scala.io.Source

object Day1 {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("input.txt").getLines().next
    val parsedData = Day1Utils.parseString(data)
    println(Day1Utils.calculate_sum(parsedData))
  }
}
