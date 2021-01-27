use std::collections::HashMap;
use std::fs;

//
// 0. charging outlet (effective rate of 0)
// 1. <- adapter <- can take from outlet 1,2 or 3 jolts lower 
// 2. device <- 3 jolts higher than the highest rated adapter 

// need to use every adapter in your bag at once, to connect from outlet to device
// calculated the difference of this long chain of adapters

fn part1() {
    let input = fs::read_to_string("input").unwrap();

    let mut adapter_joltages: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();

    adapter_joltages.sort();
    
    let mut total_difference_3 = 3;
    let mut total_difference_1 = 0;

    adapter_joltages.iter().fold(0 as usize, |acc, x| {
        let diff = *x - acc;
        if diff == 1 {
            total_difference_1 += diff;
        }
        if diff == 3 {
            total_difference_3 += diff;
        }
        *x
    });

    println!("joltages {:?}: {} {} {}", adapter_joltages, total_difference_1, total_difference_3, total_difference_1 * (total_difference_3 / 3));
}

// given an array and a root node, 
// to memoize and exclude the already encountered order, to get distinct arrangements count
fn count_ways(adapter_joltages: &Vec<usize>, index: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if memo.contains_key(&index) { return *memo.get(&index).unwrap(); }
    if index >= adapter_joltages.len() - 1 { return 1; }  

    let target_compare = adapter_joltages[index];

    let mut final_ways: usize = 0;
    //println!("current target: {:?} {:?}", target_compare, adapter_joltages);
    let last_pos = if index + 3 > adapter_joltages.len() - 1  { adapter_joltages.len() - 1} else {index + 3};
    
    let scan_range = adapter_joltages.get(index+1..=last_pos).unwrap();
    //println!("last post {} scan range {:?}", last_pos, scan_range);
    
    for (delta, joltage) in scan_range.iter().enumerate() {
        //println!("delta {} joltage {}", delta, joltage);
        if joltage - target_compare <= 3 {
            //println!("index {} target_compare {} contains : {}", index, target_compare, joltage); 
            final_ways += count_ways(adapter_joltages, index + delta + 1, memo);
        }
    }

    //println!("final return ways: {}", final_ways);
    memo.insert(index, final_ways);

    return final_ways;
}


fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut adapter_joltages: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    adapter_joltages.sort();
    adapter_joltages.insert(0, 0);
    let last = adapter_joltages[adapter_joltages.len() - 1];
    adapter_joltages.push(last + 3);

    let mut memo = HashMap::new(); 
    let number_of_ways = count_ways(&adapter_joltages, 0, &mut memo);
    println!("number of ways : {}", number_of_ways);
}





