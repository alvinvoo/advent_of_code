use std::fs;
use regex::Regex;

fn main() {

    //let input_str = "1 + (2 * 3) + (4 * (5 + 6))";
    //let input_str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    // if encounter (, create a new stack buffer, and start pushing the next
    // characters into it
    // if encounter ), take the previous stack buffer, combine with whitespace as string
    // and send to calc_given_string and return result

    // --- part 1
    //let input = fs::read_to_string("input").unwrap();

    //let ans: u64 = input.lines().map(|line| { compute(line) }).sum();

    //println!("ANS {}", ans);

    // --- part 2
    //
    //let input_str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    //let input_str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    //let input_str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    //let input_str = "(6 + 5 + 7 + 3) * (5 + ((6 + 7 + 5 + 3) + (2 * 6 + 9 + 2 * 8) + 4)) * ((7 * 7 + 8 + 8) + 4 * 7 * 8 * 2 + (4 + 5)) + (7 + (5 * 9 * 6 * 5) * 9 * (7 + 2) + 7 * (8 * 6 + 9))"; // this is wrong
    ////let input_str = "(6 + (4 * 2)) + 5 * 8";
    //let n = reorder_percedence(&input_str);
    //println!("input n=> {}", input_str);
    //let ans: u64 = compute(&n);

    let input = fs::read_to_string("input").unwrap();

    let ans: u64 = input.lines().map(|line| { 
        let n = reorder_percedence(line);
        let r = compute(&n);
        r
    }).sum();
    println!("ANS {}", ans);
    // 256432361971075 - 2nd try
    // 283582817678281 -> BINGO!
}

fn reorder_percedence(input_str: &str) -> String{
    let re = Regex::new(r"(?P<c>(\d+ \+ )+\d+)").unwrap();
    let mut after_reorder = re.replace_all(input_str, "($c)").to_string();

    let after_len = after_reorder.len();
    if after_reorder.matches("(").count() == 1 && &after_reorder[0..1] == "(" && after_reorder.matches(")").count() == 1 && &after_reorder[after_len-1..after_len] == ")" {
        after_reorder.remove(0);
        after_reorder.pop();
    }

    return after_reorder;
}


fn compute(input_str: &str) -> u64 {
    let mut buffer_str = String::new();
    let mut left_para = 0;
    for (index, c) in input_str.trim().chars().enumerate() {
        match c {
            '(' => {
                // need to tally with ) 
                // recurse? pass what's remaining
                left_para += 1;
                //println!("before compute {} this c {} left para {} input {}", buffer_str, c, left_para, input_str);
                let ans = compute(&input_str[index+1..]);
                //println!("left_para {} ans ( {}", left_para, ans);
                if (left_para == 1) {
                    buffer_str.push_str(&ans.to_string());
                }
            },
            ')' => {
                if left_para > 0 { 
                    //println!("left para before minus one {}", left_para);
                    left_para -= 1;
                    continue;
                }
                break;
            }
            _ => {
                if left_para > 0 {
                    continue;
                }
                // build string
                buffer_str.push(c);
            }
        }
    }


    let mut after_reorder = reorder_percedence(&buffer_str);

    //println!("final {}", after_reorder);

    if after_reorder.contains("(") {
        //println!("contains (");
        return compute(&after_reorder);
    }

    //println!("NOT contains (");
    return calc_given_string(&after_reorder);
}

fn calc_given_string(input_str: &str) -> u64 {
    let mut acc: u64 = 0;
    let mut operator: &str = "";
    for input in input_str.trim().split(' ') {
        let a: Result<u64, _> = input.parse();
        if a.is_ok() {
            if acc == 0 { 
                acc = a.unwrap(); 
            } else {
                if operator == "+" {
                    acc += a.unwrap();
                } else {
                    acc *= a.unwrap();
                }
            }
        } else {
            operator = input;
        }
    }

    //println!("calc givn string result {}", acc);

    acc
}
