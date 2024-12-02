use std::fs;

pub(crate) fn day2() {
    println!("Day 2");
}

fn build_vector(path: &str) -> Vec<Vec<i32>> {
    let contents =
        fs::read_to_string(path).expect("Something went wrong reading the file");
    let data = contents.split("\r\n").collect::<Vec<&str>>();

    data
        .iter()
        .map(|f| **f.split(" ")
            .collect()
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .collect()
}


fn task_1(vec: Vec<Vec<i32>>) -> i32 {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_vector_works(){
        let data = "data/day2/task1-testdata.txt";
        let vec = build_vector(data);
        assert_eq!(6, vec.len());
        assert_eq!(5, vec.first().unwrap().len());
        assert!(vec.first().unwrap().iter().all(|v| vec![7,6,4,2,1].iter().any(v)));
    }
    #[test]
    fn task1_works() {
        let list = vec![vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]];
        assert_eq!(11, task_1(list))
    }
}

