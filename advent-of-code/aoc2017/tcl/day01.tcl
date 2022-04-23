set fp [open "../input/01.txt" r]
set digits [split [string trim [read $fp]] {}]
close $fp

set l [llength $digits]

set res1 0
for {set i 1} {$i < $l} {incr i} {
    set cur [lindex $digits $i]
    set prev [lindex $digits [expr {$i - 1}]]
    if {$cur == $prev} {
        incr res1 $cur
    }
}

if {[lindex $digits 0] == [lindex $digits end]} {
    incr res1 [lindex $digits 0]
}

puts $res1


set res2 0
set h [expr {$l / 2}]
for {set i 0} {$i < $h} {incr i} {
    set current [lindex $digits $i]
    set opposite [lindex $digits [expr {$i + $h}]]
    if {$current == $opposite} {
        incr res2 $current
        incr res2 $current
    }
}

puts $res2
