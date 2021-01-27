use std::fs;

// Manhattan distance - sum of the absolute values of its east/west position and its north/south
// position
// starting position of waypoint: E10, N1
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let actions: Vec<&str> = input.lines().collect();

    let next_pos = actions.iter().fold((String::from("E10"), String::from("N1"), String::from("E0"), String::from("N0")), 
        |acc, x| {
            let (wew, wns, ew, ns) = acc;
            get_next_position_2((&wew, &wns, &ew, &ns), x)
        });

    println!("next pos {:?}", next_pos);
}

// L, R move BOTH the waypoint coordinates
// N E S W move only ONE of waypoint the coordinate
// F adds ship position to waypoint by multiple times
fn get_next_position_2(current_pos: (&str, &str, &str, &str), displacement: &str) -> (String, String, String, String) {

    let directions: Vec<&str> = vec!["N", "E", "S", "W"];

    let (direction, units) = displacement.split_at(1);

    let (way_ew_pos, way_ns_pos, ew_pos, ns_pos) = current_pos;

    let mut next_pos = (String::from(way_ew_pos), String::from(way_ns_pos), String::from(ew_pos), String::from(ns_pos));

    match direction {
        "L" => {
            let (way_ns_dir, way_ns_cur_units) = way_ns_pos.split_at(1);
            let way_ns_index: usize = directions.iter().position(|&x| x == way_ns_dir).unwrap();

            let (way_ew_dir, way_ew_cur_units) = way_ew_pos.split_at(1);
            let way_ew_index: usize = directions.iter().position(|&x| x == way_ew_dir).unwrap();

            let delta: usize = units.parse::<usize>().unwrap() / 90;
            let mut way_ns_facing: isize = way_ns_index as isize - delta as isize;
            let mut way_ew_facing: isize = way_ew_index as isize - delta as isize;

            if way_ns_facing < 0 {
                way_ns_facing += 4;
            }
            if way_ew_facing < 0 {
                way_ew_facing += 4;
            }

            // since they might switch, need to check
            // N, S
            if [0,2].contains(&way_ew_facing) {
                next_pos.1 = directions[way_ew_facing as usize].to_string() + way_ew_cur_units;
                next_pos.0 = directions[way_ns_facing as usize].to_string() + way_ns_cur_units;
            } else {
                next_pos.0 = directions[way_ew_facing as usize].to_string() + way_ew_cur_units;
                next_pos.1 = directions[way_ns_facing as usize].to_string() + way_ns_cur_units;
            }
        },
        "R" => {
            let (way_ns_dir, way_ns_cur_units) = way_ns_pos.split_at(1);
            let way_ns_index: usize = directions.iter().position(|&x| x == way_ns_dir).unwrap();

            let (way_ew_dir, way_ew_cur_units) = way_ew_pos.split_at(1);
            let way_ew_index: usize = directions.iter().position(|&x| x == way_ew_dir).unwrap();

            let delta: usize = units.parse::<usize>().unwrap() / 90;
            let mut way_ns_facing: isize = way_ns_index as isize + delta as isize;
            let mut way_ew_facing: isize = way_ew_index as isize + delta as isize;

            if way_ns_facing > 3 {
                way_ns_facing -= 4;
            }
            if way_ew_facing > 3 {
                way_ew_facing -= 4;
            }

            // since they might switch, need to check
            // N, S
            if [0,2].contains(&way_ew_facing) {
                next_pos.1 = directions[way_ew_facing as usize].to_string() + way_ew_cur_units;
                next_pos.0 = directions[way_ns_facing as usize].to_string() + way_ns_cur_units;
            } else {
                next_pos.0 = directions[way_ew_facing as usize].to_string() + way_ew_cur_units;
                next_pos.1 = directions[way_ns_facing as usize].to_string() + way_ns_cur_units;
            }

        },
        "N" => {
            let (way_ns_dir, way_ns_cur_units) = way_ns_pos.split_at(1);

            let mut ns_units = way_ns_cur_units.parse::<isize>().unwrap();

            if way_ns_dir == "N" {
                ns_units += units.parse::<isize>().unwrap();
            } else {
                ns_units = - ns_units + units.parse::<isize>().unwrap();
            }

            let final_dir = if ns_units >= 0 { "N" } else { "S" };
            let new_direction = final_dir.to_string() + &ns_units.abs().to_string();

            next_pos.1 = new_direction;
        },
        "S" => {
            let (way_ns_dir, way_ns_cur_units) = way_ns_pos.split_at(1);
            let mut ns_units = way_ns_cur_units.parse::<isize>().unwrap();

            if way_ns_dir == "S" {
                ns_units += units.parse::<isize>().unwrap();
            } else {
                ns_units = - ns_units + units.parse::<isize>().unwrap();
            }

            let final_dir = if ns_units >= 0 { "S" } else { "N" };
            let new_direction = final_dir.to_string() + &ns_units.abs().to_string();

            next_pos.1 = new_direction;
        },
        "E" => {
            let (way_ew_dir, way_ew_cur_units) = way_ew_pos.split_at(1);
            let mut ew_units = way_ew_cur_units.parse::<isize>().unwrap();

            if way_ew_dir == "E" {
                ew_units += units.parse::<isize>().unwrap();
            } else {
                ew_units = - ew_units + units.parse::<isize>().unwrap();
            }

            let final_dir = if ew_units >= 0 { "E" } else { "W" };
            let new_direction = final_dir.to_string() + &ew_units.abs().to_string();

            next_pos.0 = new_direction;
        },
        "W" => {
            let (way_ew_dir, way_ew_cur_units) = way_ew_pos.split_at(1);
            let mut ew_units = way_ew_cur_units.parse::<isize>().unwrap();

            if way_ew_dir == "W" {
                ew_units += units.parse::<isize>().unwrap();
            } else {
                ew_units = - ew_units + units.parse::<isize>().unwrap();
            }

            let final_dir = if ew_units >= 0 { "W" } else { "E" };
            let new_direction = final_dir.to_string() + &ew_units.abs().to_string();

            next_pos.0 = new_direction;
        },
        "F" => {
            let (way_ew_dir, way_ew_cur_units) = way_ew_pos.split_at(1);
            let ew_units = way_ew_cur_units.parse::<isize>().unwrap();

            let (way_ns_dir, way_ns_cur_units) = way_ns_pos.split_at(1);
            let ns_units = way_ns_cur_units.parse::<isize>().unwrap();

            let multiplier = units.parse::<isize>().unwrap();

            let new_ew_displacement = way_ew_dir.to_string() + &(ew_units * multiplier).to_string();
            let new_ns_displacement = way_ns_dir.to_string() + &(ns_units * multiplier).to_string();

            let (a, b, c, d) = get_next_position_2((ew_pos, ns_pos, way_ew_pos, way_ns_pos), &new_ew_displacement);
            let (a, b, c, d) = get_next_position_2((&a, &b, &c, &d), &new_ns_displacement);

            next_pos = (c, d, a, b);
        }
        _ => { println!("no match!"); } 
    }

    next_pos
}


// part 1
// Start position - E0, N0; Facing E
fn part1(){
    let input = fs::read_to_string("input").unwrap();
    let actions: Vec<&str> = input.lines().collect();

    //println!("actions {:?}", actions);
    
    let next_pos = actions.iter().fold((String::from("E"), String::from("E0"), String::from("N0")), 
        |acc, x| {
            let (facing, ew, ns) = acc;
            get_next_position((&facing, &ew, &ns), x)
        });
    //let next_pos = get_next_position(("S", "W2", "S1"), "F2");

    println!("next pos {:?}", next_pos);
}

fn get_next_position<'a>(current_pos: (&str, &str, &str), displacement: &str) -> (String, String, String) {
    let directions: Vec<&str> = vec!["N", "E", "S", "W"];
    //R -> forward index ; L -> backward index; changes where the ship is facing
    //N, S, E, W -> displaces the ship position
    //F -> moving forward on the facing direction

    let (direction, units) = displacement.split_at(1);

    let (facing, ew_pos, ns_pos) = current_pos;

    let facing_index: usize = directions.iter().position(|&x| x == facing).unwrap();

    let mut next_pos = (String::from(facing), String::from(ew_pos), String::from(ns_pos));

    match direction {
        "L" => {
            let delta: usize = units.parse::<usize>().unwrap() / 90;
            
            let mut new_facing: isize = facing_index as isize - delta as isize;

            if new_facing < 0 {
                new_facing += 4;
            }

            next_pos.0 = directions[new_facing as usize].to_string();
        },
        "R" => {
            let delta: usize = units.parse::<usize>().unwrap() / 90;
            
            let mut new_facing: usize = facing_index + delta;

            if new_facing > 3 {
                new_facing -= 4;
            }

            next_pos.0 = directions[new_facing as usize].to_string();
        },
        "N" => {
            let (ns_dir, ns_cur_units) = ns_pos.split_at(1);
            let mut ns_units = ns_cur_units.parse::<isize>().unwrap();

            if ns_dir == "N" {
                ns_units += units.parse::<isize>().unwrap();
            } else {
                ns_units = - ns_units + units.parse::<isize>().unwrap();
            }

            let final_dir = if ns_units >= 0 { "N" } else { "S" };
            let new_direction = final_dir.to_string() + &ns_units.abs().to_string();

            next_pos.2 = new_direction;
        },
        "S" => {
            let (ns_dir, ns_cur_units) = ns_pos.split_at(1);
            let mut ns_units = ns_cur_units.parse::<isize>().unwrap();

            if ns_dir == "S" {
                ns_units += units.parse::<isize>().unwrap();
            } else {
                ns_units = - ns_units + units.parse::<isize>().unwrap();
            }

            let final_dir = if ns_units >= 0 { "S" } else { "N" };
            let new_direction = final_dir.to_string() + &ns_units.abs().to_string();

            next_pos.2 = new_direction;
           
        },
        "E" => {
            let (ew_dir, ew_cur_units) = ew_pos.split_at(1);
            let mut ew_units = ew_cur_units.parse::<isize>().unwrap();

            if ew_dir == "E" {
                ew_units += units.parse::<isize>().unwrap();
            } else {
                ew_units = - ew_units + units.parse::<isize>().unwrap();
            }

            let final_dir = if ew_units >= 0 { "E" } else { "W" };
            let new_direction = final_dir.to_string() + &ew_units.abs().to_string();

            next_pos.1 = new_direction;
        },
        "W" => {
            let (ew_dir, ew_cur_units) = ew_pos.split_at(1);
            let mut ew_units = ew_cur_units.parse::<isize>().unwrap();

            if ew_dir == "W" {
                ew_units += units.parse::<isize>().unwrap();
            } else {
                ew_units = - ew_units + units.parse::<isize>().unwrap();
            }

            let final_dir = if ew_units >= 0 { "W" } else { "E" };
            let new_direction = final_dir.to_string() + &ew_units.abs().to_string();

            next_pos.1 = new_direction;
        },
        "F" => {
            let new_displacement = facing.to_string() + &units.to_string();
            next_pos = get_next_position(current_pos, &new_displacement);
        },
        _ => {
            println!("unknown!");
        }
    }

    next_pos

}
