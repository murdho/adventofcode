DAY2_INPUT_FILE = "../2024_inputs/day02.txt"
# DAY2_INPUT_FILE = "../2024_inputs/day02_example.txt"

def day2_part1
  reports = File.readlines(DAY2_INPUT_FILE).map { _1.split(/\s+/).map(&:to_i) }

  safe_reports = reports.filter do |levels|
    sorted = levels.sort
    next unless levels == sorted || levels == sorted.reverse

    diffs = levels.each_cons(2).map { (_1 - _2).abs }
    next unless diffs.min >= 1 && diffs.max <= 3

    true
  end

  safe_reports.count
end
