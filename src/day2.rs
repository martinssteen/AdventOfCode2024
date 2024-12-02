use std::fs;

pub(crate) fn day2() {
    let vector = build_vector("data/day2/task1.txt");
    let task_1_result = task_1(vector);

    println!("Number of safe reports: {}", task_1_result)
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

fn task_1(vec: Vec<Vec<i32>>) -> i32 {
    fn value_within_allowed_range(value: &i32) -> bool {
        (-3..4).contains(value) && *value != 0
    }
    fn value_is_positive(value: &i32) -> bool {
        value.is_positive()
    }
    fn value_is_negative(value: &i32) -> bool {
        value.is_negative()
    }

    let filteredList: Vec<_> = vec
        .iter()
        .map(|nested_vec| {
            let mut newVec = Vec::new();
            for (index, value) in nested_vec.iter().enumerate() {
                let next_index = index + 1;
                if (next_index >= nested_vec.len()) {
                    continue;
                }
                newVec.push(value - nested_vec[next_index])
            }

            newVec
        })
        .filter(|modified_vecs| {
            modified_vecs.iter().all(value_within_allowed_range)
                && (modified_vecs.iter().all(value_is_positive)
                    || modified_vecs.iter().all(value_is_negative))
        })
        .collect();

    filteredList.len() as i32
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
    fn verify_worksAsItShould() {
        let x = -3;
        assert!((-3..4).contains(&x))
    }
}
