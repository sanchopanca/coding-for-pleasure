set fd [open "../input/02.txt"]
set data [read -nonewline $fd]
close $fd

set rows_raw [split $data "\n"]
set rows {}
foreach r $rows_raw {
    lappend rows [split $r "\t"]
}

set res1 0
foreach row $rows {
    set mx [tcl::mathfunc::max {*}$row]
    set mn [tcl::mathfunc::min {*}$row]
    incr res1 [expr {$mx - $mn}]
}

puts $res1

set res2 0

foreach row $rows {
    foreach a $row {
        foreach b $row {
            if {$a != $b && ($a % $b == 0 || $b % $a == 0)} {
                if {$a % $b == 0} {
                    incr res2 [expr {$a / $b}]
                } else {
                    incr res2 [expr {$b / $a}]
                }
            }
        }
    }
}

puts [expr {$res2 / 2}]
