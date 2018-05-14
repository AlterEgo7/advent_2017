package day8

import org.scalatest.FunSpec
import Registers._

class RegistersSpec extends FunSpec {

  describe("Registers") {
    it("returns correct max register value") {
      val input = Iterator(
        "b inc 5 if a > 1",
        "a inc 1 if b < 5",
        "c dec -10 if a >= 1",
        "c inc -20 if c == 10"
      )
      assert(getMax(input) === 1)
    }
  }

}
