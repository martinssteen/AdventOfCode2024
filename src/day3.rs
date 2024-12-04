use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref mul_regex: Regex = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    static ref mul_regex_task2: Regex = Regex::new(r"mul\(\d+\,\d+\)|do\(\)|don\'t\(\)").unwrap();
    static ref digit_regex: Regex = Regex::new(r"\d+\,\d+").unwrap();
}

pub(crate) fn day3() {
    let contents =
        fs::read_to_string("data/day3/task1.txt").expect("Something went wrong reading the file");
    let task_1_result = task_1(&contents);
    let task_2_result = task_2(&contents);
    println!("Multiplication result: {}", task_1_result);
    println!("Multiplcation result (with commands): {}", task_2_result)
}

fn task_1(str: &String) -> i32 {
    //find all that matches pattern "mul(X,X)"
    mul_regex
        .find_iter(&str)
        .map(|regex_result| {
            //Find only digits and comma between
            let digits = digit_regex.find(regex_result.as_str()).unwrap().as_str();
            let split_vector = digits.split(",").collect::<Vec<&str>>();

            let first_num = split_vector.first().unwrap().parse::<i32>().unwrap();
            let last_num = split_vector.last().unwrap().parse::<i32>().unwrap();
            //finally multiply values
            first_num * last_num
        })
        .sum()
}

fn task_2(str: &String) -> i32 {
    //default activated
    let mut multiply = true;
    //find all that matches pattern "mul(X,X), or a command like do() or don't()"
    mul_regex_task2
        .find_iter(&str)
        .map(|regex_result| {
            return match regex_result.as_str() {
                "do()" => {
                    multiply = true;
                    0
                }
                "don't()" => {
                    multiply = false;
                    0
                }
                _ => {
                    if multiply {
                        //Find only digits and comma between
                        let digits = digit_regex.find(regex_result.as_str()).unwrap().as_str();
                        let split_vector = digits.split(",").collect::<Vec<&str>>();

                        let first_num = split_vector.first().unwrap().parse::<i32>().unwrap();
                        let last_num = split_vector.last().unwrap().parse::<i32>().unwrap();
                        //finally multiply values
                        first_num * last_num
                    } else {
                        0
                    }
                }
            };
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_works() {
        let contents = fs::read_to_string("data/day3/task1-testdata.txt")
            .expect("Something went wrong reading the file");
        let result = task_1(&contents);
        assert_eq!(161, result)
    }
    #[test]
    fn task2_works() {
        let contents = fs::read_to_string("data/day3/task2-testdata.txt")
            .expect("Something went wrong reading the file");
        let result = task_2(&contents);
        assert_eq!(48, result)
    }
}
