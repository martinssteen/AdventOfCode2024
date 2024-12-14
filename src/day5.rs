use std::fs;

pub(crate) fn day5() {
    let contents =
        fs::read_to_string("data/day5/day5_input.txt").expect("Something went wrong reading the file");
    let data = contents.split("\r\n").collect::<Vec<&str>>();

    let task_1_sum = task_1(data);

    println!("MIDDLE PAGES SUM: {}", task_1_sum)
}

fn task_1(vec: Vec<&str>) -> i32 {
    let split_index = vec.iter().position(|str| str.is_empty()).unwrap();
    let (rules, pages) = vec.split_at(split_index);
    //Splitting the rules into a vector of two ints
    let rules_vec: Vec<(i32, i32)> = rules
        .into_iter()
        .map(|rules| {
            let split_rule = rules.split("|").collect::<Vec<&str>>();
            (
                split_rule.first().unwrap().parse::<i32>().unwrap(),
                split_rule.last().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();
    //removing the first str which is empty
    let pages_vec = pages.split_first().unwrap().1.to_vec();

    let middle_pages = pages_vec
        .into_iter()
        //Map list of pages into ints
        .map(|str| {
            str.split(",")
                .map(|string| string.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        //Filter out all non-correct pages
        .filter(|str_list| {
            rules_vec.iter().all(|(num_1, num_2)| {
                let index_1 = str_list
                    .iter()
                    .position(|num| num == num_1);
                let index_2 = str_list
                    .iter()
                    .position(|num| num == num_2);
                (index_2.is_none() || index_1.is_none()) || (index_1.unwrap() < index_2.unwrap())
            })
        })
        .map(|str_list| {
            let middle_index = str_list.len() / 2;
            // println!("{}",            str_list[middle_index]);
            str_list[middle_index]
        })
        .collect::<Vec<i32>>();
    
    middle_pages.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_works() {
        let contents = fs::read_to_string("data/day5/task1_testdata.txt")
            .expect("Something went wrong reading the file");
        let data = contents.split("\r\n").collect::<Vec<&str>>();

        let task_1_num = task_1(data);
        assert_eq!(143, task_1_num)
    }
}
