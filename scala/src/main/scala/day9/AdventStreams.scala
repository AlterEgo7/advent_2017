package day9

object AdventStreams {
  def main(args: Array[String]): Unit = {

  }

  def getStreamScore(input: Stream[Char]): Int = {
    ???
  }

  def getStreamScore(input: String): Int = {
    getStreamScore(input.toStream)
  }
}
