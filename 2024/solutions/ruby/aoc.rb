require "active_support/all"

# Require all Ruby files from aoc/ directory
Dir.glob(File.join("aoc", "*.rb"), &method(:require_relative))

day, part = ARGV.first(2)
if day.blank? || part.blank?
  puts "usage: ruby aoc.rb DAY PART"
  exit 1
end

method_name = "day#{day}_part#{part}"
puts send method_name
