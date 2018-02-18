package day3

import org.scalatest.FunSpec
import CircularManhattan._
import CircularSums._

class Day3Spec extends FunSpec {

  describe("Day2 Part 1") {
    describe("manhattan distance of circular 2D array") {
      it("should return correct value for 1") {
        assert(manhattan(1) === 0)
      }
      it("should return correct value for 12") {
        assert(manhattan(12) === 3)
      }
      it("should return correct value for 23") {
        assert(manhattan(23) === 2)
      }
      it("should return correct value for 29") {
        assert(manhattan(29) === 4)
      }
      it("should return correct value for 49") {
        assert(manhattan(49) === 6)
      }
      it("should return correct value for 43") {
        assert(manhattan(43) === 6)
      }
      it("should return correct value for 1024") {
        assert(manhattan(1024) === 31)
      }
    }
  }

  describe("Day 3 Part 2") {
    describe("Circular sums") {
      it("should calculate sum") {
        assert(circularSum(340) === 351)
      }
    }
  }

}
