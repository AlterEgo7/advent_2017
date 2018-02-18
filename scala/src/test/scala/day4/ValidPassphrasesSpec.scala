package day4

import org.scalatest.FunSpec
import ValidPassphrases._

class ValidPassphrasesSpec extends FunSpec {

  describe("Valid Passphrases") {
    it("passphrase with distinct words should be valid") {
      assert(valid("aa bb cc dd ee") === true)
    }
    it("passphrase with duplicate words should be invalid") {
      assert(valid("aa bb cc dd aa") === false)
    }
    it("passphrase with prefixed words should be valid") {
      assert(valid("aa bb cc dd aaa") === true)
    }
  }
}
