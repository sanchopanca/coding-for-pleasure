set goal 347991

set side 1
while {$side ** 2 < $goal} {
    incr side 2
}

set last [expr {$side ** 2}]
set first [expr {($side - 2) ** 2 + 1}]

incr side -1

set pos [expr {($goal - $first) % $side + 1}]


set res1 [expr {$side / 2 + abs($pos - $side / 2)}]

puts $res1
