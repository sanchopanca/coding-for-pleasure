my $input = '../input/01.txt'.IO.slurp.chomp;
my ($x, $y) = 0, 0;
my $dir = 90;
for split(", ", $input) -> $d {
    my $m = $d.match(/(R|L)(\d+)/);
    my ($turn, $distance) = $m[0].Str, $m[1].Int;
    if $turn eq 'L' {
        $dir += 90;
    } else {
        $dir -= 90;
    }

    if $dir == 360 {
        $dir = 0;
    } elsif $dir == -90 {
        $dir = 270;
    }

    if $dir == 0 {
        $x += $distance;
    } elsif $dir == 90 {
        $y += $distance;
    } elsif $dir == 180 {
        $x -= $distance;
    } else {
        $y -= $distance;
    }
}

say $x.abs + $y.abs;

