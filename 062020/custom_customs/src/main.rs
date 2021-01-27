use std::fs;

fn part_1(){
    let input = fs::read_to_string("input").unwrap();
    let mut str_buffer = String::new();
    let mut final_count = 0;

    for line in input.lines() {
        if line == "" {
            let len = get_unique_str_len(&str_buffer);
            final_count += len;
            str_buffer.clear();
        }

        str_buffer.push_str(line);
    }
    let len = get_unique_str_len(&str_buffer);
    final_count += len;
    str_buffer.clear();

    println!("final count {}", final_count);
}

fn get_unique_str_len(input: &str) -> usize {
    let output = input.chars().fold(String::new(), |mut acc, x| { 
        if !acc.contains(x) {
            acc.push(x);
        }
        acc
    });

    output.len()
}

fn count_unique_group_answer(answers: &Vec<&str>) -> usize {
    let mut markers = vec![0;26];

    for answer in answers {
        for chr in answer.chars() {
            let chr_pos = chr as usize;
            // ascii decimal 97 'a' -> 122 'z'
            markers[chr_pos-97] += 1;
        }
    }

    let count = markers.iter().filter(|&x| *x==answers.len() as u32).count();

    //println!("count {:#?}", count);
    count
}

fn main() {
    //let y: fn(u32) -> u32 = |x| x + 1;
    //count_unique_group_answer(&vec!["abcf", "def"]);

    let input = fs::read_to_string("input").unwrap();
    let mut answers = Vec::new();
    let mut final_count = 0;

    for line in input.lines() {
        if line == "" {
            let count = count_unique_group_answer(&answers);
            final_count += count;
            answers.clear();
            continue;
        }
        answers.push(line);
    }

    let count = count_unique_group_answer(&answers);
    final_count += count;
    answers.clear();

    println!("final count {}", final_count);
}

// part 2 is about intersection within the group
// need a fn: "abc" `intersect` "zycb" -> "bc"
