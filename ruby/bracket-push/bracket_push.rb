
module Brackets
  PAIRS = {
    ')' => '(',
    ']' => '[',
    '}' => '{',
  }

  class << self
    def paired?(string)
      string.chars.reduce([]) do |stack, char|
        process_char(stack, char)
      end.empty?
    end

    private

    def process_char(stack, char)
      if closes_last_pair?(stack, char)
        stack[0...-1]
      elsif bracket?(char)
        stack + [char]
      else
        stack
      end
    end

    def closes_last_pair?(stack, char)
      closing_bracket?(char) &&
        opening_bracket_for(char) == stack.last
    end

    def bracket?(char)
      opening_bracket?(char) || closing_bracket?(char)
    end

    def opening_bracket?(char)
      PAIRS.values.include?(char)
    end

    def closing_bracket?(char)
      PAIRS.keys.include?(char)
    end

    def opening_bracket_for(bracket_char)
      PAIRS[bracket_char]
    end
  end
end

module BookKeeping
  VERSION = 4
end
