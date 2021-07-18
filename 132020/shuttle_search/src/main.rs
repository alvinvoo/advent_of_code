use std::fs;


fn part1() {
    let input = fs::read_to_string("input").unwrap();
    
    let lines: Vec<&str> = input.lines().collect();

    let earliest_timestamp: u32 = lines[0].parse().unwrap();
    let valid_bus_ids: Vec<u32> = lines[1].split(',').filter(|&x| x != "x").map(|x| x.parse().unwrap()).collect();

    println!("timestamp: {}", earliest_timestamp);
    println!("valid ids: {:?}", valid_bus_ids);

    let mut answer = (0, u32::MAX);
    for id in &valid_bus_ids {
        let denom = earliest_timestamp / id;

        let earlier_timestamp = denom * id;

        if earlier_timestamp == earliest_timestamp {
            answer = (*id, earlier_timestamp);
            break;
        }

        let (_, min_timestamp) = answer;
        if earlier_timestamp < earliest_timestamp && (earlier_timestamp + *id) < min_timestamp{
            answer = (*id, earlier_timestamp + *id);
        }
    }

    println!("answer {:?}", answer);
    println!("calc {}", (answer.1 - earliest_timestamp) * answer.0);
}

fn part2_1(){

    let input = fs::read_to_string("input").unwrap();
    
    let lines: Vec<&str> = input.lines().collect();
    let ids_xs: Vec<(usize, usize)> = lines[1].split(',')
                .enumerate()
                .filter(|(i, d)| d.parse::<usize>().is_ok())
                .map(|(i, d)| (i, d.parse::<usize>().unwrap()))
                .collect();

    let mut d: usize = 1;
    let mut i: usize = 0;
    for (offset, bus) in &ids_xs {
        while true {
            i += d;
            if (i + offset) % bus == 0 {
                d = d * bus;
                break;
            }
        }
    }

    println!("first timestamp {}", i);
}
// part 2 finding the range of denoms
fn main() {
    part2_1();
}

fn part2() {
    // can reorder the sequence so the 'index' multiplier is larger?
    //ids [7, 13, 59, 31, 19]
    //xs [1, 4, 6, 7]
    // reorder to
    //ids [59, 31, 19, 13, 7]
    //xs [+2, +3, -3, -4] 
    //
    //let ids = vec![59, 31, 19, 13, 7];
    //let xs: Vec<i64> = vec![2, 3, -3, -4];

    //ids [29, 41, 521, 23, 13, 17, 601, 37, 19]
    //xs [19, 29, 37, 42, 46, 60, 66, 79]
    //reorder to
    //
    //
    //let ids = vec![601, 521, 41, 37, 29, 23, 19, 17, 13];
    //let xs: Vec<i64> = vec![-31, -41, 6, -60, -23, 19, -14, -18];

    let ids = vec![7, 19];
    let xs: Vec<i64> = vec![7];

    let first = ids[0];
    let mut index = 2;

    let mut final_timestamps = vec![0; ids.len()];
    while final_timestamps.iter().any(|&x| x == 0) {
        let first_time = first * index;
        final_timestamps[0] = first_time;
        let mut id_index = 1;
        for x in &xs {
            if id_index > ids.len() - 1 { break; }
            let current_id = ids[id_index]; 
            let next_time = first_time + x;
            if next_time % current_id == 0 {
               final_timestamps[id_index] = next_time; 
               //println!("index {}: match id {} with {} {:?}", index, current_id, next_time, final_timestamps);
            } else {
                break;
            }
            id_index += 1;
        }

        index += 1;
    }

    println!("final timestamps {:?}", final_timestamps);

}
