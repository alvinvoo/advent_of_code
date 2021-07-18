use std::collections::HashMap;

fn main() {
    let mut numbers: Vec<u64> = vec![10,16,6,0,1,17];
    let mut memo: HashMap<u64, u64> = [(10, 1), (16, 2), (6, 3), (0, 4), (1, 5)].iter().cloned().collect();
    
    //let mut fst_pos: u64 = 0;
    //let mut snd_pos: u64 = 0;
    let mut it: u64 = 1; 
    // first number will always be the last position
    // need to memoize the number -> prev position
    while it <= (30000000 - 6) {
        //println!("memo {:?}", memo);
        //println!("numbers {:?}", numbers);
        let last_number = *numbers.last().unwrap();
        //println!("turn {} ------for last number {} ----", it + 3, last_number);
        
        let memo_ind = memo.get(&last_number);

        let mut new_ind = 0;
        if memo_ind.is_some() {
            let last_spoken_ind = memo_ind.unwrap();
            //println!("last spoken ind {}", last_spoken_ind);
            if *last_spoken_ind < numbers.len() as u64 {
                new_ind = numbers.len() as u64 - *last_spoken_ind;
            }
        }
            
        memo.insert(last_number, numbers.len() as u64);
        numbers.push(new_ind);
        //for i in (0..numbers.len()).rev() {
        //    //println!("{}", numbers[i]);

        //    if fst_pos == 0 && numbers[i] == last_number {
        //        fst_pos = (i + 1) as u64;
        //    } else if snd_pos == 0 && numbers[i] == last_number {
        //        snd_pos = (i + 1) as u64;
        //    }
        //}

        //if fst_pos > 0 && snd_pos > 0 {
        //    numbers.push(fst_pos - snd_pos);
        //} else {
        //    numbers.push(0)
        //}

        //fst_pos = 0;
        //snd_pos = 0;
        it+=1;
    }

    println!("{:?}", numbers.len());
    println!("{:?}", numbers.last());
    
}
