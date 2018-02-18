package day4

import scala.io.Source

object ValidPassphrases {
  def main(args: Array[String]): Unit = {
    val inputs = Source.fromFile("input4-1.txt")
    println(s"Part 1: ${inputs.getLines().count(valid)}")
    println(s"Part 2: ${Source.fromFile("input4-1.txt").getLines().count(validAnagram)}")
  }

  def valid(passphrase: String): Boolean = {
    val words = passphrase.split(' ')
    words.length == words.distinct.length
  }

  def validAnagram(passphrase: String): Boolean = {
    val words = passphrase.split(' ').map(_.sorted)
    words.length == words.distinct.length
  }
}