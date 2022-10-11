pub mod problems;

pub fn roman_to_int(s: String) -> i32 {
    let mut res = 0;
    let mut prev = ' ';
    let chars = s.chars();
    for c in chars {
        match c {
            'I' => res += 1,
            'V' => {
                res += 5;
                if prev == 'I' { 
                    res -= 2;
                }
            }
            'X' => {
                res += 10;
                if prev == 'I' { 
                    res -= 2;
                }
            }
            'L' => {
                res += 50;
                if prev == 'X' { 
                    res -= 20;
                }
            }
            'C' => {
                res += 100;
                if prev == 'X' { 
                    res -= 20;
                }
            }
            'D' => {
                res += 500;
                if prev == 'C' { 
                    res -= 200;
                }
            }
            'M' => {
                res += 1000;
                if prev == 'C' { 
                    res -= 200;
                }
            }
            _ => res += 0
        }

        prev = c;
    }

    res
}