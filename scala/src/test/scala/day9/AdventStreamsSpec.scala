package day9

import org.scalatest.FunSpec

class AdventStreamsSpec extends FunSpec {

  describe("AdventStreams") {
    describe("scores") {
      it("should compute score for single group") {
        assert(AdventStreams.getStreamScore("{}") == 1)
      }

      it("should compute score for nested groups") {
        assert(AdventStreams.getStreamScore("{{{}}}") == 6)
      }

      it("should compute score for nested lists of groups") {
        assert(AdventStreams.getStreamScore("{{},{}}") == 5)
      }

      it("should compute score for nested groups in different levels") {
        assert(AdventStreams.getStreamScore("{{{},{},{{}}}}") == 16)
      }

      it("should compute score of group with garbage") {
        assert(AdventStreams.getStreamScore("{<a>,<a>,<a>,<a>}") == 1)
      }

      it("should compute score of nested groups with garbage") {
        assert(AdventStreams.getStreamScore("{{<ab>},{<ab>},{<ab>},{<ab>}}") == 9)
      }

      it("should compute score of double negated signs") {
        assert(AdventStreams.getStreamScore("{{<!!>},{<!!>},{<!!>},{<!!>}}") == 9)
      }

      it("should compute score of garbage with negated closing marks") {
        assert(AdventStreams.getStreamScore("{{<a!>},{<a!>},{<a!>},{<ab>}}") == 3)
      }
    }
  }

}
