/*
There's just one problem: by holding the two lists up side by side (your puzzle input), it quickly becomes clear that the lists aren't very similar. Maybe you can help The Historians reconcile their lists?

For example:

3   4
4   3
2   5
1   3
3   9
3   3



*/
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn main() -> Result<(), Box<dyn Error>>{
    let input_path = "resources/test.txt";
    let (vec1, vec2) = read_input(input_path)?;
    let (heap1, heap2) = create_binary_heaps(vec1.clone(), vec2.clone());
    let diff = find_diff_part_1(heap1, heap2);
    println!("Diff: {}", diff);
    let score = similarity_score_part_2(vec1, vec2);
    println!("Score: {}", score);
    Ok(())
}

fn read_input(input_path: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>>{
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        let n1_str = parts.next().ok_or("Missing first number")?;
        let n2_str = parts.next().ok_or("Missing second number")?;

        let n1 = n1_str.parse::<u32>()?;
        let n2 = n2_str.parse::<u32>()?;

        l1.push(n1);
        l2.push(n2);
    }
    Ok((l1, l2))
}

fn create_binary_heaps(vec1: Vec<u32>, vec2: Vec<u32>) -> (BinaryHeap<Reverse<u32>>, BinaryHeap<Reverse<u32>>) {
    let mut heap1 = BinaryHeap::new();
    for item in vec1 {
        heap1.push(Reverse(item));
    }

    let mut heap2 = BinaryHeap::new();
    for item in vec2 {
        heap2.push(Reverse(item));
    }

    (heap1, heap2)
}

fn find_diff_part_1(mut h1: BinaryHeap<Reverse<u32>>, mut h2: BinaryHeap<Reverse<u32>>) -> u32 {
    let mut total_sum = 0;
    while !h1.is_empty() && !h2.is_empty() {
        let n1 = h1.pop().unwrap();
        let n2 = h2.pop().unwrap();
        total_sum += (n1.0).abs_diff(n2.0);
    }
    total_sum
}

fn similarity_score_part_2(v1: Vec<u32>, v2: Vec<u32>) -> u32 {
    let mut total_sum  = 0;
    let mut list2:HashMap<u32, u32> = HashMap::new();
    for num in v2 {
        list2.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }
    for num in v1 {
        if let Some(count) = list2.get(&num) {
            total_sum += count * num;
        }
    }
    total_sum
}
