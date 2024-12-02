use std::fs;

pub(crate) fn day2() {
    let vector = build_vector("data/day2/task1.txt");
    let task_1_result = task_1(vector.clone());

    let task_2_result = task_2(vector);

    println!("Number of safe reports: {}", task_1_result);
    println!(
        "Number of safe reports with Problem Dampener: {}",
        task_2_result
    )
}

fn build_vector(path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let data = contents.split("\r\n").collect::<Vec<&str>>();

    let mut vec = Vec::new();
    for i in data {
        let x = i
            .split(" ")
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        vec.push(x)
    }
    vec
}
fn value_within_allowed_range(value: &i32) -> bool {
    (-3..4).contains(value) && *value != 0
}
fn value_is_positive(value: &i32) -> bool {
    value.is_positive()
}
fn value_is_negative(value: &i32) -> bool {
    value.is_negative()
}

fn task_1(vec: Vec<Vec<i32>>) -> i32 {
    vec.iter()
        .map(|nested_vec| {
            let mut new_vec = Vec::new();
            for (index, value) in nested_vec.iter().enumerate() {
                let next_index = index + 1;
                if next_index >= nested_vec.len() {
                    continue;
                }
                new_vec.push(value - nested_vec[next_index])
            }

            new_vec
        })
        .filter(|modified_vecs| {
            modified_vecs.iter().all(value_within_allowed_range)
                && (modified_vecs.iter().all(value_is_positive)
                    || modified_vecs.iter().all(value_is_negative))
        })
        .collect::<Vec<_>>()
        .len() as i32
}

fn task_2(vec: Vec<Vec<i32>>) -> i32 {
    fn valid_list(vec: &Vec<i32>) -> bool {
        let mut new_vec = Vec::new();
        for (index, value) in vec.iter().enumerate() {
            let next_index = index + 1;
            if next_index >= vec.len() {
                continue;
            }
            new_vec.push(value - vec[next_index])
        }

        new_vec.iter().all(value_within_allowed_range)
            && (new_vec.iter().all(value_is_positive) || new_vec.iter().all(value_is_negative))
    }

    vec.iter()
        .map(|nested_vec| {
            if valid_list(nested_vec) {
                return true;
            } else {
                for i in 0..nested_vec.len() {
                    let mut clone = nested_vec.clone();
                    clone.remove(i);
                    if valid_list(&clone) {
                        return true;
                    }
                }
            }
            return false;
        })
        .filter(|x| *x)
        .collect::<Vec<_>>()
        .len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_vector_works() {
        let data = "data/day2/task1-testdata.txt";
        let vec = build_vector(data);
        assert_eq!(6, vec.len());
        assert_eq!(5, vec.first().unwrap().len());
        assert!(vec
            .first()
            .unwrap()
            .iter()
            .all(|v| vec![7, 6, 4, 2, 1].iter().any(|x| x == v)));
    }
    #[test]
    fn task1_works() {
        let data = "data/day2/task1-testdata.txt";
        let vec = build_vector(data);
        let result = task_1(vec);
        assert_eq!(2, result)
    }
    #[test]
    fn task2_works() {
        let data = "data/day2/task1-testdata.txt";
        let vec = build_vector(data);
        let result = task_2(vec);
        assert_eq!(4, result)
    }
}
