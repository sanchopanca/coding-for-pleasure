set fd [open "../input/04.txt"]
set data [read -nonewline $fd]
close $fd

set lines [split $data "\n"]
set passphrases {}
foreach l $lines {
    lappend passphrases [split $l " "]
}


set res1 0
foreach passphrase $passphrases {
    set d {}
    foreach word $passphrase {
        dict set d $word 1
    }
    set u [dict size $d]
    set l [llength $passphrase]
    if {$u == $l} {
        incr res1
    }
}

puts $res1
