use aoclib::aocinit;
use log::info;

fn process_lens(input: &str, hashmap: &mut Vec<Vec<(String, u32)>>) {
    if input.contains("=") {
        let (key, value) = input.split_once("=").unwrap();
        let value = value.parse::<u32>().unwrap();
        let keyhash = hash(key) as usize;
        if let Some(keyindex) = hashmap[keyhash].iter().position(|x| x.0 == key) {
            hashmap[keyhash][keyindex].1 = value;
        } else {
            hashmap[keyhash].push((key.to_string(), value));
        }
    } else if input.contains("-") {
        let key = &input[0..input.len()-1];
        let keyhash = hash(key) as usize;
        if let Some(keyindex) = hashmap[keyhash].iter().position(|x| x.0 == key) {
            hashmap[keyhash].remove(keyindex);
        }
    }
}

fn hash(input: &str) -> u8 {
    let mut current = 0;
    for v in input.chars() {
        current += v as u32;
        current *= 17;
        current %= 256;
    }

    return current.try_into().unwrap();
}

fn day15(input: String) {
    let init_sequence: Vec<&str> = input.split(",").collect::<Vec<&str>>();
    let result1 = init_sequence.iter().fold(0, |acc, x| acc + hash(x) as u32);
    info!("HASH sum result: {result1}");

    let mut hashmap: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    init_sequence.iter().for_each(|x| process_lens(x, &mut hashmap));
    let mut result2 = 0;
    for (i, list) in hashmap.iter().enumerate() {
        for (j, lens) in list.iter().enumerate() {
            result2 += (i+1) * (j+1) * lens.1 as usize;
        }
    }
    info!("Focussing power: {result2}");
}

fn main() {
    aocinit(day15);
}
