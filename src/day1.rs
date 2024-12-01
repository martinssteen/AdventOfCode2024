use std;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs;

pub(crate) fn day1() {
    let contents =
        fs::read_to_string("data/day1/task1.txt").expect("Something went wrong reading the file");
    let data = contents.split("\r\n").collect::<Vec<&str>>();

    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for i in data {
        let split = i.split("   ").collect::<Vec<&str>>();
        first_list.push(split.first().unwrap().parse::<i32>().unwrap());
        second_list.push(split.last().unwrap().parse::<i32>().unwrap());
    }

    task_1(first_list.clone(), second_list.clone());
    task_2(first_list.clone(), second_list.clone());
}

fn task_1(mut first_list: Vec<i32>, mut second_list: Vec<i32>) {

    first_list.sort();
    second_list.sort();
    let mut distances = Vec::new();

    for (index, value) in first_list.iter().enumerate(){
        let second_value = second_list[index];
        let distance  = (value - second_value).abs();
        distances.push(distance);
    }
    let distance_sum: i32 = distances.iter().sum();
    println!("Distance sum: {}", distance_sum)
}

fn task_2(mut first_list: Vec<i32>, mut second_list: Vec<i32>) {
    let mut similarity_list = Vec::new();

    for i in first_list {
        let list: Vec<&i32> = second_list.iter().filter(|x| **x == i).collect();
        let num = list.len() as i32;
        similarity_list.push(i * num)
    }

    let simularitySum: i32 = similarity_list.iter().sum();

    println!("Simularity Sum = {}", simularitySum)
}
