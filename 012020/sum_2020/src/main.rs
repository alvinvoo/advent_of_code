use std::fs;

fn part_1(){
    let input = fs::read_to_string("input").unwrap();

    let mut lines: Vec<usize> = input.lines().map(|str| str.parse::<usize>().unwrap()).collect();
    let mut return_number = 0;
    
    while let Some(value) = lines.pop() {
        for sec_value in &lines {
            let sum = value + sec_value;
            if sum == 2020 {
                return_number = value * sec_value;
                break;
            }
        }
    }

    println!("answer: {}", return_number);
}

fn part_2(){
    let input = fs::read_to_string("input").unwrap();

    let mut lines: Vec<usize> = input.lines().map(|str| str.parse::<usize>().unwrap()).collect();
    let mut return_number = 0;

    while let Some(value) = lines.pop() {
        for sec_value in &lines {
            let sum = value + sec_value;
            if sum >= 2020 { continue; }
            for third_value in &lines[1..] {
                let sum = value + sec_value + third_value;
                if sum == 2020 {
                    return_number = value * sec_value * third_value;
                    break;
                }
            }

            if return_number > 0 { break; }
        }
        if return_number > 0 { break; }
    }

    println!("answer: {}", return_number);
}

fn main() {
    //part_1();
    part_2();
}
