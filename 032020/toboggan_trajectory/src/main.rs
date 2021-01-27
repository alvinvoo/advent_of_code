use std::fs;
use std::str::Lines;


fn slope(input: &str, right_step: usize, down_step: usize) -> usize {
    let mut coor = 0;
    let mut total_trees = 0;

    let mut lines = input.lines();

    lines.next();

    for (index, line) in lines.enumerate() {
        if down_step > 1 && (index + 1) % down_step == 1 { continue; }
        coor += right_step;
        let c: Vec<char> = line.chars().collect();
        if coor >= c.len() {
            coor -= c.len();
        }
        if c[coor] == '#' {
            total_trees += 1;
        }
    }

    println!("total trees {}", total_trees);
    total_trees
}


fn main() {
    let input = fs::read_to_string("input").unwrap();

    let final_output = slope(&input, 1, 1) 
        * slope(&input, 3, 1)
        * slope(&input, 5, 1)
        * slope(&input, 7, 1)
        * slope(&input, 1, 2);
    
    println!("final output {}", final_output);
}
