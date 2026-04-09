use std::usize::MAX;

pub fn print_arr(v: &Vec<String>) {
    for fruit in v {
        println!("{}", fruit);
    }
}

pub fn min(v: &Vec<String>) -> usize {
    let mut min = MAX;

    for fruit_len in v {
        if fruit_len.len() < min {
            min = fruit_len.len()
        }
    }

    min
}

pub fn transpose(v: &Vec<String>) -> Vec<String> {
    let min = min(v);
    let n_fruits = v.len();
    let transposed: &mut Vec<String> = &mut vec![];

    for i in 0..n_fruits {
        let mut j = 0;
        for c in v[i].chars() {
            if j == min {
                break;
            }
            if i == 0 {
                transposed.insert(j, String::new());
            }
            transposed[j].push(c);
            j = j + 1;
        }
    }

    transposed.to_vec()
}