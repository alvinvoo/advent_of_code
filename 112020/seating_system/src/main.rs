use std::fs;

// get adjacent 8 seats
// input_vec[row_ind - 1][col_ind - 1]
// input_vec[row_ind - 1][col_ind]
// input_vec[row_ind - 1][col_ind + 1]
// input_vec[row_ind][col_ind - 1]
// input_vec[row_ind][col_ind + 1]
// input_vec[row_ind + 1][col_ind - 1]
// input_vec[row_ind + 1][col_ind]
// input_vec[row_ind + 1][col_ind + 1]
fn get_adjacent_seats_in_vec(input_vec: &Vec<Vec<char>>, row_ind: usize, col_ind: usize) -> Vec<char>{
    // assuming input vec is square
    let last_index = input_vec.len() - 1;
    let mut output_vec = Vec::new();

    if row_ind > 0 && col_ind > 0 {
        let p1 = input_vec[row_ind - 1][col_ind - 1];
        output_vec.push(p1);
    }
    if row_ind > 0 {
        let p2 = input_vec[row_ind - 1][col_ind];
        output_vec.push(p2);
    }
    if row_ind > 0 && col_ind < last_index {
        let p3 = input_vec[row_ind - 1][col_ind + 1];
        output_vec.push(p3);
    }
    if col_ind > 0 {
        let p4 = input_vec[row_ind][col_ind - 1];
        output_vec.push(p4);
    }
    if col_ind < last_index {
        let p5 = input_vec[row_ind][col_ind + 1];
        output_vec.push(p5);
    }
    if row_ind < last_index && col_ind > 0 {
        let p6 = input_vec[row_ind + 1][col_ind - 1];
        output_vec.push(p6);
    }
    if row_ind < last_index {
        let p7 = input_vec[row_ind + 1][col_ind];
        output_vec.push(p7);
    }
    if row_ind < last_index && col_ind < last_index {
        let p8 = input_vec[row_ind + 1][col_ind + 1];
        output_vec.push(p8);
    }

    output_vec.clone()
}


fn get_adjacent_direction_seats(input_vec: &Vec<Vec<char>>, pos: (isize, isize)) -> Vec<char>{
    let eight_direction_vectors = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let output_vec: Vec<char> = eight_direction_vectors.iter().map(|[a, b]| helper_one_direction(input_vec, pos, *a, *b)).collect();

    output_vec.clone()
}

fn helper_one_direction(input_vec: &Vec<Vec<char>>, pos: (isize, isize), row_dir: isize, col_dir: isize) -> char{
    let (row_ind, col_ind) = pos;     
    let last_index = (input_vec.len() - 1) as isize;
    let mut p1 = '.';
    let mut tmp_row = row_ind + row_dir;
    let mut tmp_col = col_ind + col_dir;
    while tmp_row >= 0 && tmp_col >= 0 && tmp_row <= last_index && tmp_col <= last_index
        && p1 == '.' 
    {
        p1 = input_vec[tmp_row as usize][tmp_col as usize];                           
        tmp_row += row_dir;
        tmp_col += col_dir;
    }
    p1
}

fn toggle_rules(input_vec: &Vec<Vec<char>>, toggle: bool, done: bool) -> Vec<Vec<char>> {
    if done {
        return input_vec.clone();
    }
    let mut output_vec = input_vec.clone();
    for (row_ind, mut row) in output_vec.iter_mut().enumerate() {
        for (col_ind, mut col) in row.iter_mut().enumerate() {
            match toggle {
                true => {
                    if *col =='L' && get_adjacent_seats_in_vec(&input_vec, row_ind, col_ind)
                        .iter()
                            .all(|&x| x == 'L' || x == '.')
                    {
                        *col = '#';
                    }
                },
                false => {
                    if *col =='#' && get_adjacent_seats_in_vec(&input_vec, row_ind, col_ind)
                        .iter()
                            .filter(|&x| *x == '#')
                            .count() >= 4
                    {
                        *col = 'L';
                    }
                }
            }
        }
    }
    return toggle_rules(&output_vec, !toggle, output_vec == *input_vec);
}

fn toggle_rules_2(input_vec: &Vec<Vec<char>>, toggle: bool, done: bool) -> Vec<Vec<char>> {
    if done {
        return input_vec.clone();
    }
    let mut output_vec = input_vec.clone();
    for (row_ind, mut row) in output_vec.iter_mut().enumerate() {
        for (col_ind, mut col) in row.iter_mut().enumerate() {
            match toggle {
                true => {
                    if *col =='L' && get_adjacent_direction_seats(&input_vec, (row_ind as isize, col_ind as isize))
                        .iter()
                            .all(|&x| x == 'L' || x == '.')
                    {
                        *col = '#';
                    }
                },
                false => {
                    if *col =='#' && get_adjacent_direction_seats(&input_vec, (row_ind as isize, col_ind as isize))
                        .iter()
                            .filter(|&x| *x == '#')
                            .count() >= 5 
                    {
                        *col = 'L';
                    }
                }
            }
        }
    }
    return toggle_rules_2(&output_vec, !toggle, output_vec == *input_vec);
}
fn part1(){
    let input = fs::read_to_string("input").unwrap();
    let mut input_vec: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let line_row: Vec<char> = line.chars().collect();
        input_vec.push(line_row);
    }

    //println!("before input vec {:?}", input_vec);
    let final_vec: Vec<Vec<char>> = toggle_rules(&input_vec, true, false);

    //println!("final vec {:?}", final_vec);

    let final_sum: usize = final_vec.iter().map(|row| row.iter().filter(|&x| *x=='#').count()).sum();
    println!("final sum {}", final_sum);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut input_vec: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let line_row: Vec<char> = line.chars().collect();
        input_vec.push(line_row);
    }
    
    //let y = get_adjacent_direction_seats(&input_vec, (3, 3));

    //println!("y {:?}", y);

    let final_vec: Vec<Vec<char>> = toggle_rules_2(&input_vec, true, false);

    let final_sum: usize = final_vec.iter().map(|row| row.iter().filter(|&x| *x=='#').count()).sum();
    println!("final sum {}", final_sum);

    //
    //
    // the 2 rules execute one after another at once
    //let mut output_vec = input_vec.clone();
    //for (row_ind, mut row) in output_vec.iter_mut().enumerate() {
    //    for (col_ind, mut col) in row.iter_mut().enumerate() {
    //        if *col =='L' && get_adjacent_direction_seats(&input_vec, (row_ind as
    //        isize, col_ind as isize))
    //                    .iter()
    //                    .all(|&x| x == 'L' || x == '.')
    //        {
    //            *col = '#';
    //        }
    //    }
    //}

    //println!("after output_vec vec {:?}", output_vec);
    //input_vec = output_vec.clone();
    //println!("after input_vec vec {:?}", input_vec);
    //for (row_ind, mut row) in output_vec.iter_mut().enumerate() {
    //    for (col_ind, mut col) in row.iter_mut().enumerate() {
    //        if *col =='#' && get_adjacent_direction_seats(&input_vec, (row_ind as
    //      isize, col_ind as isize))
    //                .iter()
    //                .filter(|&x| *x == '#')
    //                .count() >= 5
    //        {
    //            *col = 'L';
    //        }
    //    }
    //}
    //println!("testing equality {:?}", input_vec == output_vec);
    //println!("after2 output_vec vec {:?}", output_vec);


    //input_vec = output_vec.clone();
    //for (row_ind, mut row) in output_vec.iter_mut().enumerate() {
    //    for (col_ind, mut col) in row.iter_mut().enumerate() {
    //        if *col =='L' && get_adjacent_direction_seats(&input_vec, (row_ind as
    //        isize, col_ind as isize))
    //                    .iter()
    //                    .all(|&x| x == 'L' || x == '.')
    //        {
    //            *col = '#';
    //        }
    //    }
    //}

    //println!("after3 output_vec vec {:?}", output_vec);
}


