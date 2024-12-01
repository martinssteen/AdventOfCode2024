use std;
use std::cmp::Ordering;
use std::env;
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
    fn find_smallest(vec: &Vec<i32>) -> (usize, &i32) {
        vec.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap()
    }

    let mut distances = Vec::new();

    while (!first_list.iter().all(|&x| x == 999999999)) {
        let (first_index, value) = find_smallest(&first_list);
        let (second_index, value2) = find_smallest(&second_list);

        let distance = (value - value2).abs();
        // println!("FIRST VAL | INDEX = {} , {} | SECOND = {}, {} | DISTANCE = {} ", value, first_index, value2, second_index, distance);
        distances.push(distance);
        first_list[first_index] = 999999999;
        second_list[second_index] = 999999999;
    }

    let distanceSum: i32 = distances.iter().sum();
    println!("Distance sum: {}", distanceSum)
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
