use std::fs;
use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    static ref mul_regex : Regex = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    static ref digit_regex: Regex = Regex::new(r"\d+\,\d+").unwrap();
}


fn task_1(str: String)-> i32 {
    //find all that matches pattern "mul(X,X)"
    mul_regex.find_iter(&str)
        .map(|regex_result| {
            //Find only digits and comma between
            let digits = digit_regex.find(regex_result.as_str()).unwrap().as_str();
            let split_vector = digits.split(",").collect::<Vec<&str>>();

            let first_num = split_vector.first().unwrap().parse::<i32>().unwrap();
            let last_num = split_vector.last().unwrap().parse::<i32>().unwrap();
            //finally multiply values
            first_num * last_num
        }).sum()
}


pub(crate) fn day3() {
    let contents =
        fs::read_to_string("data/day3/task1.txt").expect("Something went wrong reading the file");
    let task_1_result = task_1(contents);
    println!("Multiplication result: {}", task_1_result)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_works() {
        let contents =
            fs::read_to_string("data/day3/task1-testdata.txt").expect("Something went wrong reading the file");
        let result = task_1(contents);
        assert_eq!(161, result)
    }
}