use std::fs;

fn get_parent_bags<'a>(input: &'a str, input_bags: &Vec<&str>, output_bags: &mut Vec<&'a str>) -> Vec<&'a str> {
    for input_bag in input_bags { // if input_bags is empty, will just skip this loop
        let output_bags_in: Vec<&str> = input.lines()
            .map(|line| {
                line.split(" bags contain").collect::<Vec<&str>>()
            })
            .filter(|parts| {
                parts[1].contains(input_bag) 
            })
            .map(|parts| {
                parts[0]
            })
            .collect();
        let mut temp_in = output_bags_in.clone();

        output_bags.append(&mut temp_in);

        //println!("input bag {:?} output_bags {:?} output_bags_in {:?}", input_bag, output_bags, output_bags_in);

        get_parent_bags(input, &output_bags_in, output_bags);
    }

    output_bags.clone()
}

fn part_1() {
    let input = fs::read_to_string("input").unwrap();    
    let mut output_bags = Vec::new();
    let ret_bags: Vec<&str> = get_parent_bags(&input, &vec!["shiny gold"], &mut output_bags);

    let final_bags = ret_bags.iter().fold(vec![], |mut acc, x| {
        if !acc.contains(x) {
            acc.push(x);
        }
        acc
    });

    println!("final bags count {:?} count: {}", final_bags, final_bags.len());
}

fn get_child_bags(input: &str, input_bag: &str, total_count: &mut u32) -> u32 {
    let child_bags: Vec<&str> = input.lines()
        .map(|line| {
            line.split(" bags contain ").collect::<Vec<&str>>()
        })
        .filter(|parts| {
            parts[0].contains(input_bag) 
        })
        .map(|parts| {
            parts[1].split(", ").collect::<Vec<&str>>()
        })
        .flatten()
        .collect();

    for bag in &child_bags{
        let bag_tokens: Vec<&str> = bag.split_whitespace().collect();
        if let Ok(bag_count) = bag_tokens[0].parse::<u32>() {
            let bag_name = bag_tokens[1].to_string() + " " + bag_tokens[2];
            let mut new_count = 0;
            *total_count += bag_count + bag_count * get_child_bags(input, &bag_name, &mut new_count);
        }
    }

    *total_count
}

fn main() {
    let input = fs::read_to_string("input").unwrap();    

    let mut total_count = 0;
    println!("child bags count {}", get_child_bags(&input, "shiny gold", &mut total_count));
}
