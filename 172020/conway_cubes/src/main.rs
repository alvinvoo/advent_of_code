use std::fs;
use std::convert::TryInto;

fn change_state(cubes2: &mut Vec<Vec<char>>){
    // 26 position shifts
    // 8 * 3 diff z's position + own position * 2 diff z's pos (up and down)
    // col, row
    let z_u1: Vec<(isize, isize)> = vec![(-1, -1),(-1, 0),(-1, 1),(0, -1),(0, 0),(0, 1),(1, -1),(1, 0),(1, 1)];
    let z_0: Vec<(isize, isize)> = vec![(-1, -1),(-1, 0),(-1, 1),(0, -1),(0, 1),(1, -1),(1, 0),(1, 1)];
    let z_l1: Vec<(isize, isize)> = vec![(-1, -1),(-1, 0),(-1, 1),(0, -1),(0, 0),(0, 1),(1, -1),(1, 0),(1, 1)];

    let ref_cubes2 = cubes2.clone();
    for (i, col) in cubes2.iter_mut().enumerate() {
        for (j, row) in col.iter_mut().enumerate() {
            // get all valid neighbors pos, return count of '#'
            let i = i as isize;
            let j = j as isize;
            let h_count = z_0.iter().filter_map(|(c_s, r_s)| {
                let l_c_s: Result<usize, _> = (c_s + i).try_into();
                let l_r_s: Result<usize, _> = (r_s + j).try_into();
                if l_c_s.is_ok() && l_r_s.is_ok() { // means >=0
                    let t: Option<&Vec<char>> = ref_cubes2.get(l_c_s.unwrap());
                    if t.is_some() && t.unwrap().get(l_r_s.unwrap()) == Some(&'#') {
                        return Some(1);
                    }
                }
                None
            })
            .count();

            //println!("for {} at pos {:?}, active neighbors# {}", row, (i, j), h_count);

            if row == &'#' && h_count != 2 && h_count != 3 {
                // if neighbor count is not 2 and not 3, change to '.'
                *row = '.';
            } else if h_count == 3 {
                // if neighbor count is 3, change to '#'
                *row = '#';
            }
        }
    }
}

fn change_state_3D(cubes3: &mut Vec<Vec<Vec<char>>>){
    // 26 position shifts
    // 8 * 3 diff z's position + own position * 2 diff z's pos (up and down)
    // col, row
    let z_u1: Vec<(isize, isize, isize)> = vec![(-1, -1, -1),(-1, -1, 0),(-1, -1, 1),(-1, 0, -1),(-1, 0, 0),(-1, 0, 1),(-1, 1, -1),(-1, 1, 0),(-1, 1, 1)];
    let z_0: Vec<(isize, isize, isize)> = vec![(0, -1, -1),(0, -1, 0),(0, -1, 1),(0, 0, -1),(0, 0, 1),(0, 1, -1),(0, 1, 0),(0, 1, 1)];
    let z_l1: Vec<(isize, isize, isize)> = vec![(1, -1, -1),(1, -1, 0),(1, -1, 1),(1, 0, -1),(1, 0, 0),(1, 0, 1),(1, 1, -1),(1, 1, 0),(1, 1, 1)];

    let last_index = cubes3.len() - 1;
    let ref_cubes3 = cubes3.clone();
    //println!("length {}", cubes3.len());
    for (l, cubes2) in cubes3.iter_mut().enumerate() {

        // depending on which l, different position shifts
        let l_z = if l == 0 { vec![&z_0, &z_l1] } 
        else if l == last_index { vec![&z_u1, &z_0] } 
        else { vec![&z_u1, &z_0, &z_l1] };

        let l_z_all: Vec<(isize, isize, isize)> = l_z.into_iter().cloned().flatten().collect();

        for (i, col) in cubes2.iter_mut().enumerate() {
            for (j, row) in col.iter_mut().enumerate() {
                // get all valid neighbors pos, return count of '#'
                let l = l as isize;
                let i = i as isize;
                let j = j as isize;
                let h_count = l_z_all.iter().filter_map(|(l_s, c_s, r_s)| {
                    let l_l_s: Result<usize, _> = (l_s + l).try_into(); // this should always be ok
                    let l_c_s: Result<usize, _> = (c_s + i).try_into();
                    let l_r_s: Result<usize, _> = (r_s + j).try_into();
                    if l_l_s.is_ok() && l_c_s.is_ok() && l_r_s.is_ok() { // means >=0
                        let cubes2: Option<&Vec<Vec<char>>> = ref_cubes3.get(l_l_s.unwrap());
                        if cubes2.is_some() {
                            let t: Option<&Vec<char>> = cubes2.unwrap().get(l_c_s.unwrap());
                            if t.is_some() && t.unwrap().get(l_r_s.unwrap()) == Some(&'#') {
                                return Some(1);
                            }
                        }
                    }
                    None
                })
                .count();

                //println!("for {} at pos {:?}, active neighbors# {}", row, (l, i, j), h_count);

                if row == &'#' && h_count != 2 && h_count != 3 {
                    // if neighbor count is not 2 and not 3, change to '.'
                    *row = '.';
                } else if h_count == 3 {
                    // if neighbor count is 3, change to '#'
                    *row = '#';
                }
            }
        }
    }
    // after everything is done, duplicate the last layer as the first layer
    cubes3.insert(0, cubes3[last_index].clone());
}
fn transform_neighbors_2D(cubes2: &mut Vec<Vec<char>>){
    // first pad the 'outer layer' with .
    cubes2.iter_mut().for_each(|x: &mut Vec<char>| {x.insert(0, '.'); x.push('.');});
    let current_len = cubes2[0].len();
    let outer_paddings = vec!['.'; current_len];
    cubes2.insert(0, outer_paddings.clone());
    cubes2.push(outer_paddings);
}

fn transform_neighbors_3D(cubes3: &mut Vec<Vec<Vec<char>>>){
    // first pad the 'outer layer' with .
    for cubes2 in cubes3.iter_mut() {
        transform_neighbors_2D(cubes2);
    }
    // now, insert a dummy . 2D layers into cubes3
    cubes3.push(vec![vec!['.'; cubes3[0].len()]; cubes3[0].len()]);
}

fn part1() {
    let input = fs::read_to_string("input").unwrap();

    let mut v_cubes3: Vec<Vec<Vec<char>>> = Vec::new();
    let mut v_cubes2: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        v_cubes2.push(line.chars().collect());
    }

    v_cubes3.push(v_cubes2);

    //println!("cubes {:?}", v_cubes2);
    //transform_neighbors_2D(&mut v_cubes2);
    //println!("1.cubes {:?}", v_cubes2);
    //change_state(&mut v_cubes2);
    //println!("2.cubes {:?}", v_cubes2);

    // strategy for one cycle:
    // 1. check size of outer array - Z length 
    // 2. do 3D padding  (just add one more empty layer on top would be enough?)
    // 3. do 2D padding 
    // 4. do change_state for Z = 0, Z = -1 until Z = -n
    // needs to check for all 3 layers IF available
    // 5. update Z = 1 by copying Z = -1 .. until Z = -n

    // part 1 - 3D
    for i in 0..6 {
        transform_neighbors_3D(&mut v_cubes3);
        change_state_3D(&mut v_cubes3);
    }
    let final_count = v_cubes3.into_iter().flatten().flatten().filter(|&x| x == '#').count();

    println!("final count {}", final_count);
}

fn transform_neighbors_4D(cubes4: &mut Vec<Vec<Vec<Vec<char>>>>){
    for cubes3 in cubes4.iter_mut() {
        // first pad all existing 2D outer layers with .
        for cubes2 in cubes3.iter_mut() {
            transform_neighbors_2D(cubes2);
        }
        // each 3D layer needs to pad above and below
        cubes3.insert(0, vec![vec!['.'; cubes3[0].len()]; cubes3[0].len()]);
        cubes3.push(vec![vec!['.'; cubes3[0].len()]; cubes3[0].len()]);
    }

    let c3_len = cubes4[0][0].len();
    let c4_len = cubes4[0].len();

    let dummy_3D_matrix = vec![vec![vec!['.'; c3_len]; c3_len]; c4_len];

    // the 4D layer needs to pad above and below
    cubes4.insert(0, dummy_3D_matrix.clone());
    cubes4.push(dummy_3D_matrix);
}

fn main(){
    let input = fs::read_to_string("input2").unwrap();

    let mut v_cubes2: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        v_cubes2.push(line.chars().collect());
    }

    let mut v_cubes4: Vec<Vec<Vec<Vec<char>>>> = Vec::new();

    // we can reason about cube4 being a 2 * 2 matrix (which contains a 2D matrix) by itself
    v_cubes4 = vec![vec![v_cubes2]];

    // in every iteration we first pad (4 * cycle) of dummy 2D matrix layers
    // cycle 1:
    // [[ ] [ ] [ ]  
    //  [ ] [0] [1]
    //  [2] [3] [4]]
    // cycle 2:
    // [[ ]   [ ]   [ ]   [ ]  [ ]  
    //  [ ]   [ ]   [0]   [1]  [2-1]
    //  [2-2] [2]   [3]   [4]  [2-3]
    //  [2-4] [2-5] [2-6] [2-7][2-8]]
    //  ...
    //  and then, we transform the 0 layer + the padded layers and mirror them
    //  back to the upper left corner to complete one cycle
    //
    //
    //println!("cubes4 {:?}", v_cubes4);
    //transform_neighbors_4D(&mut v_cubes4);
    //println!("1.cubes4 {:?}", v_cubes4);
    // we only need to consider the following positions for each cycle, for the half lower right
    // portion of the matrices
    // cycle 1: (1,0), (1,1)
    // cycle 2: (2,0), (2,1) (2,2) (1,2) + prev cycle 1 permutation
    // cycle 3: (3,0), (3,1) (3,2) (3,3) (2,3) (1,3)  + prev cycle 2+ cycle 1 permutation
    // ...
    // these positions are already filled up by the transform function, basically its all the 
    // positiosn to the RHS of the 0,0
    // cycle 1:
    // [[ ] [ ] [ ]  
    //  [ ] [0] [x]
    //  [ ] [ ] [x]]
    // cycle 2:
    // [[ ]  [ ]  [ ]  [ ]  [ ]  
    //  [ ]  [ ]  [ ]  [ ]  [ ]  
    //  [ ]  [ ]  [0]  [x]  [x]
    //  [ ]  [ ]  [ ]  [x]  [x]
    //  [ ]  [ ]  [ ]  [x]  [x]]
    //  ...
    // once a position is calculated, it needs to mirror to 4 other positions (they are the same!)
    // the generic formula to mirror is: (x,y) (-y,x) (-x,-y) (y,x), for e.g. (2,0) will mirror to
    // (0,2) (0,-2) (-2,0); (1,2) will mirror to (-2,1) (-1,-2) (2,1)
    //calc_state_4D(&mut v_cubes4);
    //println!("2.cubes4 {:?}", v_cubes4);
    //transform_neighbors_4D(&mut v_cubes4);
    //println!("3.cubes4 {:?}", v_cubes4);
    //calc_state_4D(&mut v_cubes4);
    //println!("4.cubes4 {:?}", v_cubes4);

    for i in 0..6 {
        transform_neighbors_4D(&mut v_cubes4);
        calc_state_4D(&mut v_cubes4);
    }
    let final_count = v_cubes4.into_iter().flatten().flatten().flatten().filter(|&x| x == '#').count();

    println!("final count {}", final_count);
}

fn change_one_state_4D(cubes4_ref: &Vec<Vec<Vec<Vec<char>>>>, cubes4: &mut Vec<Vec<Vec<Vec<char>>>>, w: usize, z: usize) {
    let all_neighbors = get_4D_neighbors();
    // calc the original point
    let cubes2 = &mut cubes4[w][z];
    for (i, col) in cubes2.iter_mut().enumerate() {
        for (j, row) in col.iter_mut().enumerate() {
            let h_count = all_neighbors.iter().filter_map(|(w_s, z_s, y_s, x_s)| {
                let w_r: Result<usize, _> = (w_s + w as isize).try_into();
                let z_r: Result<usize, _> = (z_s + z as isize).try_into();
                let y_r: Result<usize, _> = (y_s + i as isize).try_into();
                let x_r: Result<usize, _> = (x_s + j as isize).try_into();
                if w_r.is_ok() && z_r.is_ok() && y_r.is_ok() && x_r.is_ok() { // means >=0
                    let cubes3: Option<&Vec<Vec<Vec<char>>>> = cubes4_ref.get(w_r.unwrap());
                    if cubes3.is_some() {
                        let cubes2: Option<&Vec<Vec<char>>> = cubes3.unwrap().get(z_r.unwrap());
                        if cubes2.is_some() {
                            let t: Option<&Vec<char>> = cubes2.unwrap().get(y_r.unwrap());
                            if t.is_some() && t.unwrap().get(x_r.unwrap()) == Some(&'#') {
                                return Some(1);
                            }
                        }
                    }
                }
                None
            })
            .count();

            //println!("for {} at pos {:?}, active neighbors# {}", row, (l, i, j), h_count);

            if row == &'#' && h_count != 2 && h_count != 3 {
                // if neighbor count is not 2 and not 3, change to '.'
                *row = '.';
            } else if h_count == 3 {
                // if neighbor count is 3, change to '#'
                *row = '#';
            }
        }
    }
}

fn calc_state_4D(mut cubes4: &mut Vec<Vec<Vec<Vec<char>>>>){
    // check the mid-point index   
    let full_len = cubes4.len();
    let mid_point_ind = full_len / 2;

    let w = mid_point_ind;
    let z = mid_point_ind;

    let cubes4_ref = cubes4.clone();

    change_one_state_4D(&cubes4_ref, &mut cubes4, w, z);
        
    // calc the additional points which need to be mirrored 4 times
    for z_s in (mid_point_ind + 1)..full_len {
        for w_s in mid_point_ind..full_len {
            //println!("{} {}", z_s, w_s);
            change_one_state_4D(&cubes4_ref, &mut cubes4, w_s, z_s);
            // mirror (w,z) 3 times to (-z,w) (-w,-z) (z,-w) - all relative to mid,mid
            // first shirt everything to 0,0
            let z_d = z_s - mid_point_ind;
            let w_d = w_s - mid_point_ind;

            cubes4[mid_point_ind-z_d][w_d+mid_point_ind] = cubes4[w_s][z_s].clone();
            cubes4[mid_point_ind-w_d][mid_point_ind-z_d] = cubes4[w_s][z_s].clone();
            cubes4[z_d+mid_point_ind][mid_point_ind-w_d] = cubes4[w_s][z_s].clone();
        }
    }
}

fn get_4D_neighbors() -> Vec<(isize, isize, isize, isize)>{
    // 3 * 3 * 3 * 3 permutations of (p, p, p, p) where p in {-1, 0, 1}
    // except position (0,0,0,0)
    
    let mut ret_v = Vec::new();
    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if w==0 && z==0 && y==0 && x==0 { continue; }
                    ret_v.push((w, z, y, x));
                }
            }
        }
    }

    ret_v
}
