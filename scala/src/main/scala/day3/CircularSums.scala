package day3

import scala.collection.mutable.ArrayBuffer
import scala.math.{ceil, sqrt}

object CircularSums {
  def main(args: Array[String]): Unit = {
    val input = 347991

  }

  def circularSum(input: Long): Long = {
    val size = ceil(sqrt(input)).toInt / 2 + 1
    val data = ArrayBuffer.fill(size, size)(0)
    -1
  }

  def calcCircularSum(rowIndex: Int, columnIndex: Int, cachedValues: ArrayBuffer[ArrayBuffer[Int]]): Long = {
    val cachedValue = cachedValues(rowIndex)(columnIndex)
    0
  }
}
