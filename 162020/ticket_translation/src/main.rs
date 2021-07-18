use std::fs;
use regex::Regex;
use std::ops::Range;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let rule = fs::read_to_string("rules").unwrap();
    let ticket = fs::read_to_string("tickets").unwrap();

    let re_ranges = Regex::new(r"(?P<title>.*): (?P<lr1>\d+)-(?P<ur1>\d+) or (?P<lr2>\d+)-(?P<ur2>\d+)").unwrap();

    let mut all_rules: HashMap<String, Vec<Range<u64>>> = HashMap::new();
    let mut all_ranges: Vec<Range<u64>> = Vec::new();
    for line in rule.lines() {
        let caps = re_ranges.captures(line).unwrap();
        let range1 = *&caps["lr1"].parse::<u64>().unwrap()..*&caps["ur1"].parse::<u64>().unwrap() + 1;
        let range2 = *&caps["lr2"].parse::<u64>().unwrap()..*&caps["ur2"].parse::<u64>().unwrap() + 1;

        //println!("{:?}", range1);
        //println!("{:?}", range2);

        all_ranges.push(range1.clone());
        all_ranges.push(range2.clone());
        // ORDER OF HASHMAP is NOT FIXED!
        all_rules.insert(caps["title"].to_string(), vec!(range1, range2));
    }

    //println!("ranges {:?}", all_ranges);
    //println!("rules {:?}", all_rules);

    // part 1
    //let ticket_error_rate: u64 = ticket.lines()
    //    .filter_map(|line| {
    //        let l_i: Vec<u64> = line.split(',').map(|l| l.parse::<u64>().unwrap()).collect();
    //        let t = get_invalid_number_if_any(&all_ranges, l_i);
    //        if t == 0 { None } else { Some(t) }
    //    })
    //    .sum();
    //println!("lines {}", ticket_error_rate);

    let valid_tickets: Vec<Vec<u64>> = ticket.lines()
        .filter_map(|line| {
            let l_i: Vec<u64> = line.split(',').map(|l| l.parse::<u64>().unwrap()).collect();
            let t = get_invalid_number_if_any(&all_ranges, &l_i);
            if t == 0 { Some(l_i) } else { None }
        })
    .collect();

    //println!("valid tickets {:?}", valid_tickets);
    //println!("col size {}", valid_tickets[0].len());
    //println!("row size {}", valid_tickets.len());

    let mut cats: Vec<Vec<&str>> = Vec::new();
    for col_i in 0..valid_tickets[0].len() {
        let mut row_cats: Vec<Vec<&str>> = Vec::new();
        for row_i in 0..valid_tickets.len() {
            let v = valid_tickets[row_i][col_i];
            //println!("col {} row {} value {}", col_i, row_i, v);

            let mut temp_v: Vec<&str> = Vec::new();
            for (title, ranges) in all_rules.iter() {
                if ranges[0].contains(&v) || ranges[1].contains(&v) {
                    if !temp_v.contains(&title.as_str()) {temp_v.push(&title);}
                }
            }
            row_cats.push(temp_v);
        }

        //println!("BEFORE row_cats: {:?}", row_cats);

        let mut s = row_cats.iter().fold(row_cats[0].clone(), |acc, x| {
            let a: HashSet<&str> = acc.into_iter().collect();
            let b: HashSet<_> = x.iter().cloned().collect();

            a.intersection(&b).cloned().collect()
        });

        s.sort();

        println!("{:?}", s);

        //WRONG! cannot store only the first one, can be multiple categories HIT!
        //cats.push(s.clone());
        row_cats.clear();
        // remove s from the rules list
        // all_rules.remove(&s[0].to_string());
        // ANSWER AT output3 file
    }

    //println!("cats {:?}", cats);

    //println!("row cats {:?}", row_cats);

    //let a: HashSet<u32> = vec![1, 2, 3].into_iter().collect();
    //let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();

    //let mut intersection: Vec<_> = a.intersection(&b).collect();

    //println!("{:?}", intersection[0]);

    //let mult_d = vec![vec![1, 2, 3], vec![4, 3, 2, 5], vec![6, 5, 3, 2]];
    //let s = mult_d.iter().fold(mult_d[0].clone(), |acc, x| {
    //    let a: HashSet<u32> = acc.into_iter().collect();
    //    let b: HashSet<_> = x.iter().cloned().collect();

    //    a.intersection(&b).cloned().collect()
    //});

    //println!("s? {:?}", s);
}

// want a lambda function, which, given a vector, return the NUMBER when encounter FALSE in
// all_ranges, else return ZERO, 0
fn get_invalid_number_if_any(all_ranges: &Vec<Range<u64>>, x_v: &Vec<u64>)->u64 {
    for x in x_v {
        if !all_ranges.iter().any(|r| r.contains(&x)) {
            return *x;
        }
    }
    0
}

