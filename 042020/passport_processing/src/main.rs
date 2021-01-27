use std::fs;
use regex::Regex;

fn check_valid_fields(fields: &Vec<&str>) -> bool{
    for field in fields {
        let sub_fields: Vec<&str> =  field.split(':').collect();
        let key = sub_fields[0];
        let value = sub_fields[1];

        match key {
            "byr" => {
                let byr: usize = value.parse().unwrap();
                if byr < 1920 || byr > 2002 { 
                    return false; 
                }
            },
            "iyr" => {
                let iyr: usize = value.parse().unwrap();
                if iyr < 2010 || iyr > 2020 { 
                    return false; 
                }
            },
            "eyr" => {
                let eyr: usize = value.parse().unwrap();
                if eyr < 2020 || eyr > 2030 { 
                    return false; 
                }
            },
            "hgt" => {
                if value.len()-2 == 0 { return false; }
                let unit = &value[value.len()-2..];
                let measurement: usize = value[..value.len()-2].parse().unwrap();

                if unit == "cm" &&  measurement >= 150 && measurement <= 193 {
                    continue;
                }
                if unit == "in" &&  measurement >= 59 && measurement <= 76 {
                    continue;
                }

                return false;
            },
            "hcl" => {
                let re = Regex::new("#([0-9]|[a-f]){6}").unwrap();
                if re.is_match(value) {
                    //println!("match! {}", value);
                    continue;
                }

                return false;
            },
            "ecl" => {
                let valid_colors = vec!["amb","blu","brn","gry","grn","hzl","oth"];
                if value.len() == 3 && valid_colors.contains(&value) {
                    continue;
                }

                return false;
            },
            "pid" => {
                let re = Regex::new("[0-9]{9}").unwrap();  
                if re.is_match(value) {
                    continue;
                }

                return false;
            },
            "cid" => {
            },
            _ => {
                println!("do nothing");
            }
        }
    }

    true
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut current_line = String::new();
    let mut passports: Vec<String> = Vec::new();
    for line in input.lines() {
        if line == "" {
            passports.push(current_line.clone().trim().to_string());
            current_line.clear(); 
        }
        else { 
            current_line.push(' ');
            current_line.push_str(line);
        } 
    }
    // the last line
    passports.push(current_line.clone().trim().to_string());

    //println!("passport len {}", passports.len());
    //println!("passport {:#?}", passports);

    //fields criteria:
    //1. <7 - false
    //2. =7 - have cid - false, dont have cid - true
    //3. >7 (=8) = true
    let mut total_valid = 0;
    let mut passed_passports = Vec::new();
    for passport in &passports {
        let fields: Vec<&str> = passport.split(' ').collect();
        if fields.len() < 7 { continue; }
        if fields.len() > 7 { 
            if check_valid_fields(&fields) {
                total_valid += 1; 
                passed_passports.push(passport);
                continue; 
            };
        }
        if fields.len() == 7 {
            let mut increase = true;
            for field in &fields {
                let sub_fields: Vec<&str> =  field.split(':').collect();
                if sub_fields[0] == "cid" { 
                    increase = false;
                    break;
                }
            }
            if increase { 
                if check_valid_fields(&fields) {
                    total_valid += 1; 
                    passed_passports.push(passport);
                };
            }
        }
    }


    println!("total valids: {}", total_valid);
    //let h = "help";
    //println!("testing {}", &h[h.len()-2..]);
    //println!("testing {}", &h[..h.len()-2]);
    println!("{:#?}", passed_passports);
}
