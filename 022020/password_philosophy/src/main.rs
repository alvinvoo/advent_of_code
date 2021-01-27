use std::fs;

fn part_1(){
    let input = fs::read_to_string("input").unwrap();
    let mut correct_counter = 0;

    for line in input.lines() {
        let v: Vec<&str> = line.split(": ").collect();
        let rules: Vec<&str> = v[0].split(' ').collect();
        let password: &str = v[1];

        let bounds: Vec<&str> = rules[0].split('-').collect();
        let target_char: &str = rules[1];

        let lower_bound: usize = bounds[0].parse().unwrap();
        let upper_bound: usize = bounds[1].parse().unwrap();

        let matches_len: usize = password.matches(target_char).collect::<Vec<&str>>().len();

        println!("lower {} upper {} matches {}", lower_bound, upper_bound, matches_len);

        if matches_len >= lower_bound && matches_len <= upper_bound {
            correct_counter +=1;
        }
    }

    println!("correct_counter {}", correct_counter);
}

fn part_2(){
    let input = fs::read_to_string("input").unwrap();
    let mut counter = 0;

    for line in input.lines() {
        let v: Vec<&str> = line.split(": ").collect();
        let rules: Vec<&str> = v[0].split(' ').collect();
        let password: &str = v[1];

        let bounds: Vec<&str> = rules[0].split('-').collect();
        let target_char: char = rules[1].parse().unwrap();

        let first_pos: usize = bounds[0].parse::<usize>().unwrap() - 1;
        let second_pos: usize = bounds[1].parse::<usize>().unwrap() - 1;

        let pass_c: Vec<char> = password.chars().collect();
        print!("\ntarget {} first pos {} second pos {} password {}",target_char, first_pos, second_pos, password);

        if first_pos > password.len() || second_pos > password.len() { 
            println!(" outof bounds");
            continue; 
        } 
        if (pass_c[first_pos] == target_char) ^ (pass_c[second_pos] == target_char) {
            println!(" valid");
            counter += 1;
        }
    }

    println!("counter {}", counter);
}

fn main() {
    // XOR
    //println!(" {} ", true ^ true);
    //println!(" {} ", true ^ false);
    //println!(" {} ", false ^ true);
    //println!(" {} ", false ^ false);
    part_2();
}
