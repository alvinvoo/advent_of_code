use std::fs;

/*
 * F -> lower half (changes the upper bound)
 * B -> upper half (changes the lower bound)
 *
 * L - lower half (changes upper_bound)
 * R - upper_half (changes lower bound)
 */

fn get_binary_value(input: &str) -> Option<u32>{
    let mut lower_bound = 0;
    let mut upper_bound = 2u32.pow(input.len() as u32) - 1;

    let input_chars: Vec<char> = input.chars().collect();

    for ch in &input_chars {
        match ch {
            'F' | 'L' => {
                upper_bound = upper_bound - ((upper_bound - lower_bound + 1) / 2);
            },
            'B' | 'R' => {
                lower_bound = ((upper_bound - lower_bound + 1) / 2) + lower_bound;
            },
            _ => {}
        }
    }

    if lower_bound == upper_bound {
        return Some(lower_bound);
    }

    None
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut plane_map = vec![vec![false; 8];128];
    let max = input.lines()
        .map(|line| {
            let (row_str, col_str) = line.split_at(7);
            let row_no = get_binary_value(row_str).unwrap();
            let col_no = get_binary_value(col_str).unwrap();

            plane_map[row_no as usize][col_no as usize] = true;
            let ret = row_no * 8 + col_no;
            //println!("row {}, col {}, ret {}", row_no, col_no, ret);
            ret
        })
        .max()
        .unwrap();

    println!("max {}", max);
    //println!("{:#?}", plane_map);

    for (index, row) in plane_map.iter().enumerate() {
        let false_count = row.iter().filter(|&col| !col).count();
        if false_count == 1{
            println!("{}: {:#?}", index, row);
            break;
        }
    }

    // pt 2 - find which is the missing boarding pass from the list
    // seat ID +1 -1 should be true (and front and back +8 -8 as well)
    // ignoring the very front or back rows, where there are all false
    //
    // row 64 col 5 = 517 
}
