using StringTools;

class Day04 {
    static public function main(): Void {
        part2();
    }
    static function part2() {
        var result = 0;
        var passPhrases = getPassPhrases();
        for (passPhrase in passPhrases) {
            var pass = [];
            for (word in passPhrase) {
                pass.push(new Set(word));
            }
            var break_ = false;
            for (i in 0...(pass.length - 1)) {
                for (j in (i+1)...pass.length) {
                    if (pass[i].equals(pass[j])) {
                        break_ = true;
                        break;
                    }
                }
                if (break_) {
                    break;
                }

            }
            if (!break_) {
                result += 1;
            }
        }
        trace(result);
    }

    static function getPassPhrases(): Array<Array<String>> {
        var passPhrases = [];
        for (line in sys.io.File.getContent('../input/04.txt').rtrim().split("\n")) {
            passPhrases.push(line.split(" "));
        }
        return passPhrases;
    }
}

class Set {
    private var map: Map<Int, Bool>;

    public function new(s: String) {
        map = new Map<Int, Bool>();
        for (c in s.iterator()) {
            map[c] = true;
        }
    }

    public function len(): Int {
        return Lambda.count(map);
    }

    public function equals(other: Set): Bool {
        if (this.len() != other.len()) {
            return false;
        }

        for (key => value in map) {
            if (other.map[key] != value) {
                return false;
            }
        }
        return true;
    }
}