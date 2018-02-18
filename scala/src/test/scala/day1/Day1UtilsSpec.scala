package day1

import org.scalatest.FunSpec

class Day1UtilsSpec extends FunSpec {

  describe("Day1 Part 2") {
    describe("summing function") {
      it("should sum correctly when all elements participate") {
        assert(Day1Utils.calculate_sum(Array(1,2,1,2)) === 6)
      }
      it("should sum correctly when no elements participate") {
        assert(Day1Utils.calculate_sum(Array(1,2,2,1)) === 0)
      }
      it("should sum correctly when one element participates") {
        assert(Day1Utils.calculate_sum(Array(1,2,3,4,2,5)) === 4)
      }
      it("should sum correctly when multiple elements participate") {
        assert(Day1Utils.calculate_sum(Array(1,2,3,1,2,3)) === 12)
        assert(Day1Utils.calculate_sum(Array(1,2,1,3,1,4,1,5)) === 4)
      }
    }
  }

}
