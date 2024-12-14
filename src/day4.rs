use std;
use std::fs;
pub(crate) fn day4() {
    let contents =
        fs::read_to_string("data/day4/task1.txt").expect("Something went wrong reading the file");
    let data = contents.split("\r\n").collect::<Vec<&str>>();

    let task_1_num = task_1(data.clone());
    let task_2_num = task_2(data);
    println!("NUM XMAS'es : {}", task_1_num);
    print!("NUM X-MAS'es : {}", task_2_num)
}

fn task_1(vec: Vec<&str>) -> i32 {
    let mut sum = 0;
    //Horizontal
    for str in vec.iter() {
        sum += find_all_matches(str.to_string());
    }
    for (index, _) in vec.iter().enumerate() {
        let mut vert_vec = Vec::new();
        for i in 0..vec.len() {
            vert_vec.push(vec[i].chars().nth(index).unwrap());
        }

        let vec_matches = find_all_matches(vert_vec.iter().collect());

        sum += vec_matches
    }

    for x in (0..vec.len()).rev() {
        let mut diagonal_vec: Vec<char> = Vec::new();
        for i in 0..vec.len() {
            // println!("VEC INDEX: {}, CHAR INDEX: {}", x, i);
            if x + i >= vec.len() {
                continue;
            }
            let next_char = vec[x + i].chars().nth(i);
            if next_char.is_some() {
                diagonal_vec.push(next_char.unwrap());
            }
        }
        sum += find_all_matches(diagonal_vec.into_iter().collect::<String>());
    }
    for x in 1..vec.len() {
        let mut diagonal_vec: Vec<char> = Vec::new();
        for i in 0..vec.len() {
            let next_char = vec[i].chars().nth(i + x);
            if next_char.is_some() {
                diagonal_vec.push(next_char.unwrap());
            }
        }

        sum += find_all_matches(diagonal_vec.into_iter().collect::<String>());
    }
    let reversed = vec
        .iter()
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    for x in (0..reversed.len()).rev() {
        let mut diagonal_vec: Vec<char> = Vec::new();
        for i in 0..vec.len() {
            // println!("VEC INDEX: {}, CHAR INDEX: {}", x, i);
            if x + i >= vec.len() {
                continue;
            }
            let next_char = reversed[x + i].chars().nth(i);
            if next_char.is_some() {
                diagonal_vec.push(next_char.unwrap());
            }
        }
        sum += find_all_matches(diagonal_vec.into_iter().collect::<String>());
    }
    for x in 1..reversed.len() {
        let mut diagonal_vec: Vec<char> = Vec::new();
        for i in 0..vec.len() {
            let next_char = reversed[i].chars().nth(i + x);
            if next_char.is_some() {
                diagonal_vec.push(next_char.unwrap());
            }
        }
        sum += find_all_matches(diagonal_vec.into_iter().collect::<String>());
    }

    sum
}

fn find_all_matches(string: String) -> i32 {
    let num = string.matches("XMAS").count();
    let num_reversed = string.matches("SAMX").count();
    // println!("STRING {} | NUM: {} | REVERSED: {}", string, num,num_reversed);
    return (num + num_reversed) as i32;
}

fn is_mas_or_sam(string: String) -> bool {
    string == "MAS" || string == "SAM"
}

fn task_2(vec: Vec<&str>) -> i32 {
    let mut a_char_indexes: Vec<(usize, Vec<usize>)> = Vec::new();
    let mut count = 0;
    // Find all A characters
    for (index, vec) in vec.iter().enumerate() {
        if index != 0 && index != vec.len() - 1 {}
        let all_a_char_indexes: Vec<usize> = vec
            .chars()
            .into_iter()
            .enumerate()
            .filter(|x| {
                let (_, char) = *x;
                char == 'A'
            })
            .map(|(index, _)| index)
            .collect();
        a_char_indexes.push((index, all_a_char_indexes))
    }
    
    for (vec_index, a_list) in a_char_indexes.iter() {
        for index in a_list.iter() {
            //if within original vector
            if *vec_index != 0 && vec_index + 1 != vec.len() {
                let str = vec[*vec_index];
                if *index != 0 && index + 1 != str.len() {
                    let top_left = vec[vec_index - 1].chars().nth(index - 1).unwrap();
                    let top_right = vec[vec_index - 1].chars().nth(index + 1).unwrap();

                    let bottom_left = vec[vec_index + 1].chars().nth(index - 1).unwrap();
                    let bottom_right = vec[vec_index + 1].chars().nth(index + 1).unwrap();

                    let str1 = vec![top_left, 'A', bottom_right]
                        .into_iter()
                        .collect::<String>();
                    let str2 = vec![top_right, 'A', bottom_left]
                        .into_iter()
                        .collect::<String>();
                    
                    if is_mas_or_sam(str1) && is_mas_or_sam(str2) {
                        count += 1
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_matches_works() {
        let under_test = "XMASSAKDASODKSAKDOSAMXSSAMXXMASLOLFKDSOKFDSFSDXMAS";
        let result = find_all_matches(under_test.to_string());
        assert_eq!(5, result)
    }
    
    #[test]
    fn equals_char(){
        let char = 'b';
        assert!('b' == char)
    }

    #[test]
    fn task1_works() {
        let contents = fs::read_to_string("data/day4/task1-testdata2.txt")
            .expect("Something went wrong reading the file");
        let data = contents.split("\r\n").collect::<Vec<&str>>();

        assert_eq!(18, task_1(data))
    }

    #[test]
    fn task2_works() {
        let contents = fs::read_to_string("data/day4/task2-testdata.txt")
            .expect("Something went wrong reading the file");
        let data = contents.split("\r\n").collect::<Vec<&str>>();

        assert_eq!(9, task_2(data))
    }
}
