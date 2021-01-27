use std::fs;

// infinite loop: The moment the program tries to run any instruction a second time, you know it will never terminate.
//
fn part1() {
    let input = fs::read_to_string("input").unwrap();
    let instructions: Vec<&str> = input.lines().collect(); 
    
    let mut stack = vec![0;instructions.len()];
    let mut counter: i32 = 0;
    let mut prev_accumulator: i32 = 0;
    let mut accumulator: i32 = 0;

    //println!("plus 1 {:?}", "+1".parse::<i32>());
    //println!("minus 1 {:?}", "-1".parse::<i32>());

    while stack.iter().find(|&&x| x >= 2).is_none() {
        //println!("executing..");
        let instruct_value: Vec<&str> = instructions[counter as usize]
                            .split_whitespace()
                            .collect();
        match instruct_value[0] {
            "acc" => {
                stack[counter as usize] += 1;
                prev_accumulator = accumulator;
                accumulator += instruct_value[1].parse::<i32>().unwrap();
            },
            "jmp" => {
                stack[counter as usize] += 1;
                counter += instruct_value[1].parse::<i32>().unwrap();
                continue;
            },
            "nop" | _ => {},
        }
        
        counter +=1;
    }

    println!("prev {} accumulator {}", prev_accumulator, accumulator);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let instructions: Vec<&str> = input.lines().collect(); 

    //let jmp_instr_indexes: Vec<usize> = instructions.iter()
    //    .enumerate()
    //    .filter(|(index, instruction)| {
    //    let instruct_value: Vec<&str> = instruction.split_whitespace().collect();
    //    instruct_value[0] == "jmp"
    //    })
    //    .map(|(index, _)|{
    //        index
    //    })
    //    .collect();

    //println!("jmp instr {:?}", jmp_instr_indexes);
    
    //std::process::exit(0);
    //let FINAL_POS = instructions.len() - 1;
    let jmp_positions: Vec<i32> = vec![0, 3, 6, 8, 9, 13, 18, 21, 24, 27, 28, 32, 33, 35, 39, 42, 44, 46, 47, 50, 51, 55, 56, 57, 58, 62, 66, 69, 70, 71, 73, 75, 76, 77, 79, 82, 84, 85, 87, 88, 92, 96, 97, 99, 103, 105, 110, 112, 115, 116, 117, 121, 122, 123, 126, 127, 132, 136, 137, 138, 142, 145, 149, 151, 152, 157, 160, 162, 163, 165, 170, 173, 178, 181, 182, 185, 189, 193, 198, 203, 206, 209, 214, 217, 220, 221, 222, 223, 226, 228, 230, 231, 235, 237, 240, 241, 243, 244, 247, 248, 250, 255, 260, 264, 267, 268, 270, 273, 277, 282, 284, 289, 293, 295, 298, 299, 303, 304, 305, 307, 309, 313, 315, 318, 321, 323, 327, 328, 331, 334, 336, 339, 342, 347, 348, 352, 354, 356, 358, 359, 362, 363, 367, 369, 370, 371, 376, 380, 384, 387, 392, 397, 402, 405, 406, 410, 412, 413, 415, 417, 419, 421, 423, 425, 430, 434, 435, 436, 439, 440, 445, 450, 451, 455, 456, 460, 462, 463, 467, 469, 470, 474, 477, 478, 482, 485, 486, 489, 493, 497, 499, 503, 504, 507, 510, 514, 517, 519, 521, 524, 527, 530, 533, 538, 542, 543, 545, 546, 548, 551, 555, 558, 559, 564, 569, 570, 571, 576, 577, 582, 587, 588, 590, 592, 596, 601, 602, 604];

    // testing for jmp (to nop) 
    for jmp_pos in &jmp_positions {
        let mut stack = vec![0;instructions.len()];
        let mut counter: i32 = 0;
        let mut prev_accumulator: i32 = 0;
        let mut accumulator: i32 = 0;
        //println!("testing for jmp pos {}", jmp_pos);
        while stack.iter().find(|&&x| x >= 2).is_none() {
            //println!("executing..");
            let instruct_value: Vec<&str> = instructions[counter as usize]
                .split_whitespace()
                .collect();
            match instruct_value[0] {
                "acc" => {
                    stack[counter as usize] += 1;
                    prev_accumulator = accumulator;
                    accumulator += instruct_value[1].parse::<i32>().unwrap();
                },
                "jmp" => {
                    if counter != (*jmp_pos) as i32 {
                        stack[counter as usize] += 1;
                        counter += instruct_value[1].parse::<i32>().unwrap();
                        if counter >= instructions.len() as i32 {
                            println!("FOUND!");
                            println!("prev {} accumulator {}", prev_accumulator, accumulator);
                            break;
                        }
                        continue;
                    }
                },
                "nop" | _ => {},
            }

            counter +=1;
            if counter >= instructions.len() as i32 {
                println!("FOUND!");
                println!("prev {} accumulator {}", prev_accumulator, accumulator);
                break;
            }
        }
    }
}
