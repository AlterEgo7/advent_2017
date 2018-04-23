package day7

sealed trait Tree[+A] {
  val value: A
  var weight: Int
  def total_weight: Int

  override def hashCode(): Int = value.hashCode()
}

case class Leaf[A](value: A, var weight: Int) extends Tree[A] {
  override def total_weight: Int = weight
}

case class Node[A](value: A, var weight: Int, children: Seq[Tree[A]]) extends Tree[A] {
  private[this] var cached_weight: Int = _

  def total_weight: Int = {
    cached_weight match {
      case 0 =>
        cached_weight = children.map(n => n.total_weight).sum + weight
        cached_weight
      case _ =>
        cached_weight
    }
  }
}

