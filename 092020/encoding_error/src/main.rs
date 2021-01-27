use std::fs;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

// part 1 is a canSum problem
fn can_sum_memo(targetSum: isize, numbers: &[isize], memo: &mut HashMap<isize, bool>) -> bool {
    if memo.contains_key(&targetSum) {
        return memo[&targetSum]
    }
    if targetSum == 0 { return true; }
    if targetSum < 0 { return false; }

    for num in numbers {
        let remainder = targetSum - num;
        if can_sum_memo(remainder, numbers, memo) == true {

            memo.insert(targetSum, true);
            return true;
        }
    }

    memo.insert(targetSum, false);
    false
}

fn part1() {
    let input = fs::read_to_string("input").unwrap();    

    let mut input_vec: Vec<isize> = input.lines().map(|line| line.parse::<isize>().unwrap()).collect();
    let preamble_length = 25;
    //let mut somevec = vec![35, 20, 15, 25, 47];
    //somevec.sort_by(|a,b| b.partial_cmp(a).unwrap());
    while let Some(sub_vec) = input_vec.get(0..=(preamble_length + 1)) {
        let mut memo = HashMap::new();
        let (sum, target) = sub_vec.split_at(preamble_length);
        let c = can_sum_memo(target[0], sum, &mut memo);
        if !c {
            println!("target found {}", target[0]);
            break;
        }
        input_vec.remove(0);
    }

    // part 1 answer: 2089807806
}

// part 2 is a howSum problem
// twist: must be contiguous set

fn main() {
    let now = SystemTime::now();

    let input = fs::read_to_string("input").unwrap();    

    let mut input_vec: Vec<isize> = input.lines().map(|line| line.parse::<isize>().unwrap()).collect();
    let mut target_sum = 2089807806;
    let mut sum = 0;
    let mut index = 0;
    let mut target_vec = Vec::new();

    while sum != target_sum {
        target_vec.push(input_vec[index]);
        sum += input_vec[index];

        if sum > target_sum {
            input_vec.remove(0);
            target_vec.clear();
            sum = 0;
            index = 0;
        }

        index += 1;
    }

    println!("target vec {:?}", target_vec);
    target_vec.sort();
    println!("sorted target vec {:?}", target_vec);


    println!("elapse {}", now.elapsed().unwrap().as_micros());
}
