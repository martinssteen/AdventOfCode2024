use std;
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

    let distance = task_1(first_list.clone(), second_list.clone());
    let similarity = task_2(first_list.clone(), second_list.clone());

    println!("DISTANCE SUM: {}", distance);
    println!("SIMILARITY SUM: {}", similarity);

}

fn task_1(mut first_list: Vec<i32>, mut second_list: Vec<i32>) -> i32 {

    first_list.sort();
    second_list.sort();
    let mut distances = Vec::new();

    for (index, value) in first_list.iter().enumerate(){
        let second_value = second_list[index];
        let distance  = (value - second_value).abs();
        distances.push(distance);
    }
    return distances.iter().sum();
}

fn task_2(first_list: Vec<i32>, second_list: Vec<i32>) -> i32{
    let mut similarity_list = Vec::new();

    for i in first_list {
        let list: Vec<&i32> = second_list.iter().filter(|x| **x == i).collect();
        let num = list.len() as i32;
        similarity_list.push(i * num)
    }

    return similarity_list.iter().sum();

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_works(){
        let list_1 = vec![3,4,2,1,3,3];
        let list_2 = vec![4,3,5,3,9,3];
        assert_eq!(11, task_1(list_1,list_2))
    }
    #[test]
    fn task2_works(){
        let list_1 = vec![3,4,2,1,3,3];
        let list_2 = vec![4,3,5,3,9,3];
        assert_eq!(31, task_2(list_1,list_2))
    }
    
}