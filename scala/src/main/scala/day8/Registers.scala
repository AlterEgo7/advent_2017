package day8

import scala.collection.immutable.HashMap
import scala.io.Source
import scala.util.matching.Regex

object Registers {

  type RegisterState = HashMap[String, Int]

  def main(args: Array[String]): Unit = {
    println(getMax(Source.fromFile("input8.txt").getLines()))
  }

  def getMax(operations: Iterator[String]): (Int, Int) = {
    val registerState = operations.foldLeft((new RegisterState, Int.MinValue)){ case ((acc, globalMax), fullOperation) =>
      val Array(operationInput, conditionInput) = fullOperation.split(" if ")

      if (condition(acc, conditionInput)) {
        operation(acc, globalMax, operationInput)
      } else {
        (acc, globalMax)
      }
    }

    (registerState._1.values.max, registerState._2)
  }

  def condition(registers: RegisterState, input: String): Boolean = {
    val regex = new Regex("""([a-z]+) ([!<>=]{1,2}) (-?\d+)""", "register", "operation", "operand")
    val matches = regex.findFirstMatchIn(input).get

    matches.group("operation") match {
      case "==" => registers.getOrElse(matches.group("register"), 0) == matches.group("operand").toInt
      case "<"  => registers.getOrElse(matches.group("register"), 0) < matches.group("operand").toInt
      case "<=" => registers.getOrElse(matches.group("register"), 0) <= matches.group("operand").toInt
      case ">"  => registers.getOrElse(matches.group("register"), 0) > matches.group("operand").toInt
      case ">=" => registers.getOrElse(matches.group("register"), 0) >= matches.group("operand").toInt
      case "!=" => registers.getOrElse(matches.group("register"), 0) != matches.group("operand").toInt
      case _    => throw new IllegalArgumentException(s"Bad condition operation: ${matches.group("operation")}")
    }
  }

  def operation(acc: RegisterState, globalMax: Int, input: String): (RegisterState, Int) = {
    val regex = new Regex("""([a-z]+) (inc|dec) (-?\d+)""", "register", "operation", "operand")
    val matches = regex.findFirstMatchIn(input).get
    val currentValue = acc.getOrElse(matches.group("register"), 0)

    matches.group("operation") match {
      case "inc" =>
        val newValue = currentValue + matches.group("operand").toInt
        (acc + (matches.group("register") -> newValue), List(newValue, globalMax).max)
      case "dec" =>
        val newValue = currentValue - matches.group("operand").toInt
        (acc + (matches.group("register") -> newValue), List(newValue, globalMax).max)
      case _ => throw new IllegalArgumentException(s"Bad operation operation: ${matches.group("operation")}")
    }
  }
}
