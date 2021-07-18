use std::collections::HashMap;
use std::fs;
use regex::Regex;
use regex::Captures;

fn part1() {
    let input = fs::read_to_string("input").unwrap();
    let mut final_map: HashMap<String, usize> =  HashMap::new();

    let mut bitmask = String::new();
    for line in input.lines() {
       let line_parts: Vec<&str> = line.split(" = ").collect(); 
       let action = line_parts[0];
       let value = line_parts[1];

       if action == "mask" {
           bitmask = value.to_string(); 
       } else {
           let re = Regex::new(r"mem\[(\d+)\]").unwrap();
           let cap = re.captures(action).unwrap();
           //println!("{:#?}", &cap[1]);
           //println!("bitmask {:?}", bitmask);

           let v: usize = value.parse().unwrap();
           let mut input: String = format!("{:0>36}",format!("{:b}", v));

           for (ind, c) in bitmask.chars().enumerate() {
               if c == 'X' {
                   continue;
               }

               input.replace_range(ind..ind+1, &c.to_string());
           }

           final_map.insert(cap[1].to_string(), usize::from_str_radix(&input, 2).unwrap());
       }
    }

    let total_values: usize = final_map.values().sum();
    println!("final HM {:?}", total_values);
}

fn main() {
    part2();
}

fn pad_zeros(s: &str, t: usize) -> String{
    let l = s.len();

    println!("s {}, size {}", s, t);

    let d = t - l;

    if d <= 0 { return s.to_string() }
    (0..d-1).fold("0".to_string(), |acc, _| acc + "0") + s
}

fn part2() {
    let input = fs::read_to_string("input").unwrap();
    let mut final_map: HashMap<usize, usize> =  HashMap::new();

    let mut bitmask = String::new();
    for line in input.lines() {
       let line_parts: Vec<&str> = line.split(" = ").collect(); 
       let action = line_parts[0];
       let value = line_parts[1];

       if action == "mask" {
           bitmask = value.to_string(); 
       } else {
           let re = Regex::new(r"mem\[(\d+)\]").unwrap();
           let cap = re.captures(action).unwrap();

           //println!("mem {:?}", mem_addr);

           let v: usize = cap[1].parse().unwrap();
           let mut mem_addr_bin: String = format!("{:0>36}",format!("{:b}", v));
           let mut num_x: usize = 0;

           for (ind, c) in bitmask.chars().enumerate() {
               if c == '0' {
                   continue;
               }
               if c == 'X' {
                   num_x += 1;
               }
               mem_addr_bin.replace_range(ind..ind+1, &c.to_string());
           }

           //println!("{}", mem_addr_bin);

           let re = Regex::new(r"X").unwrap();
           for p in 0..2_u32.pow(num_x as u32) {
               let mut chg_input = mem_addr_bin.clone();
               let s: Vec<char> = pad_zeros(&format!("{:b}",p), num_x).chars().collect();
               //let s: Vec<char> = format!("{:0>2}", format!("{:b}",p)).chars().collect();

               for c in s.iter() {
                   chg_input = re.replace(&chg_input, |_: &Captures|{c.to_string()}).to_string();
               }

                let vv: usize = value.trim().parse().unwrap();
                //println!("inserting {} -> {}", chg_input, vv);
               final_map.insert(usize::from_str_radix(&chg_input, 2).unwrap(), vv);
           }
       }
    }

    let total_values: usize = final_map.values().sum();
    println!("final HM {:?}", total_values);
    //let v = 42;
    //let mut input: String = format!("{:0>36}",format!("{:b}", v));
    //let bitmask = "000000000000000000000000000000X1001X";
    //// do bitwise or |, unless it X

    //for (ind, c) in bitmask.chars().enumerate() {
    //    if c == '0' {
    //        continue;
    //    }
    //    input.replace_range(ind..ind+1, &c.to_string());
    //}

    //println!("{}", input);

    //let mut mem_vec: Vec<String> = Vec::new();
    //let num_x = 2;
    //let re = Regex::new(r"X").unwrap();
    //for p in 0..2_u32.pow(num_x) {
    //    let mut chg_input = input.clone();
    //    let s: Vec<char> = format!("{:0>2}", format!("{:b}",p)).chars().collect();

    //    for c in s.iter() {
    //        chg_input = re.replace(&chg_input, |_: &Captures|{c.to_string()}).to_string();
    //    }

    //    usize::from_str_radix(&chg_input, 2).unwrap()
    //    mem_vec.push(chg_input);
    //}

    //println!("all vec {:?}", mem_vec);

}
