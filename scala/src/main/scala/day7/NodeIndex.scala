package day7

import scala.collection.mutable

case class NodeIndex[A] (var map: mutable.HashMap[A, Tree[A]], var forest: mutable.Set[Tree[A]]) {
  def this() = {
    this(new mutable.HashMap[A, Tree[A]], new mutable.HashSet[Tree[A]])
  }
}
