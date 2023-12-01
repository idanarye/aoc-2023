def "main" [] {}

let $input = open -r $env.input | split row "\n"

def "main part1" [] {
    $input | each { |line|
        let digits = $line | parse -r '(?P<d>\d)' | get d
        if ($digits | is-empty) {
            return
        }
        $"($digits | first)($digits | last)" | into int
    } | math sum
}

def "main part2" [] {
    $input | each { |line|
        if ($line | is-empty) {
            return
        }
        let first_digit = $line | parse -r '(?P<d>\d|one|two|three|four|five|six|seven|eight|nine).*' | get d | get 0
        let last_digit = $line | parse -r '.*(?P<d>\d|one|two|three|four|five|six|seven|eight|nine)' | get d | get 0
        let digits = [$first_digit $last_digit] | each { |digit|
            match $digit {
                "one" => 1
                "two" => 2
                "three" => 3
                "four" => 4
                "five" => 5
                "six" => 6
                "seven" => 7
                "eight" => 8
                "nine" => 9
                _ => ($digit | into int)
            }
        }
        if ($digits | is-empty) {
            return
        }
        ($digits | first) * 10 + ($digits | last)
    } | math sum
}
