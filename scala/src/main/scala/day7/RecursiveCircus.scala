package day7

import scala.io.Source
import scala.util.matching.Regex.Match

object RecursiveCircus {
  def main(args: Array[String]): Unit = {
    val file = Source.fromFile("input7.txt")
    val nodeIndex = new NodeIndex[String]
    for (line <- file.getLines()) {
      parse_tree(line, nodeIndex)
//      println(s"Processing: ${line.split(" ").head} | Running forest size: ${nodeIndex.forest.size}")
    }
    println(nodeIndex.forest.size)
    nodeIndex.forest.foreach(x => println(x.value))
  }

  private def parse_tree(input: String, nodeIndex: NodeIndex[String]) = {
    val pattern = """(\w+)\s\((\d+)\)(\s->\s)?(.+)?""".r
    val matches: Match = pattern.findFirstMatchIn(input).get

    matches.group(4) match {
      case null =>
        create_leaf(matches.group(1), matches.group(2).toInt, nodeIndex)
      case (children_string) =>
        val children = children_string.split(", ")
        create_node(matches.group(1), matches.group(2).toInt, children.toSeq, nodeIndex)
    }

  }

  private def create_leaf(value: String, weight: Int, nodeIndex: NodeIndex[String]): Tree[String] = {
    nodeIndex.map.get(value) match {
      case Some(node) =>
        if (weight != 0) {
          node.weight = weight
        }
        node
      case None =>
        val leaf = Leaf(value, weight)
        nodeIndex.map(value) = leaf
        nodeIndex.forest.add(leaf)
        leaf
    }
  }

  private def create_node(value: String, weight: Int, children: Seq[String], nodeIndex: NodeIndex[String]): Tree[String] = {
    val children_nodes = children.map(x => create_leaf(x, 0, nodeIndex))
    nodeIndex.map.get(value) match {
      case Some(node) =>
        nodeIndex.map(value) = Node(value, weight, children_nodes)
        nodeIndex.forest --= children_nodes
        node
      case None =>
        val node = Node(value, weight, children_nodes)
        nodeIndex.map(value) = node
        nodeIndex.forest --= children_nodes
        nodeIndex.forest.add(node)
        node
    }
  }
}
