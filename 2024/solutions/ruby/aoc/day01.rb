DAY1_INPUT_FILE = "../../inputs/day01.txt"
# DAY1_INPUT_FILE = "../../inputs/day01_example.txt"

def day1_part1
  read_lists(DAY1_INPUT_FILE)
    .map(&:sort)
    .then { _1.zip _2 }
    .map { (_1 - _2).abs }
    .sum
end

def day1_part2
  left, frequencies = read_lists(DAY1_INPUT_FILE).then { [_1, _2.tally] }
  left.sum { _1 * frequencies.fetch(_1, 0) }
end

def read_lists(input_file)
  File.readlines(input_file)
      .map { _1.split(/\s+/).map(&:to_i) }
      .transpose
end
