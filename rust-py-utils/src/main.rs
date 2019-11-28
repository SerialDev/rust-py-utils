use std::collections::HashMap;

// _ _ _ _ _ _ _ _ _ _ _ _    /¯¯¯ List Utils ¯¯¯\_ _ _ _ _ _ _ _ _ _ _ _    //

pub fn uniquify(data: &Vec<i64>) -> Vec<i64> {
    let mut seen: HashMap<i64, i64> = HashMap::new();
    let mut result: Vec<i64> = Vec::new();
    for item in data.iter() {
        if !seen.contains_key(item) {
            seen.insert(*item, *item);
            result.push(*item);
        }
        println!("{}", item);
    }

    return result;
}

pub fn flatten_tuple_sequence(tuple_seq: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    for row in tuple_seq {
        result.push(row.0);
        result.push(row.1);
    }

    return result;
}

// // TODO: Fix
// pub fn flatten_tuple_sequenc_generic<T>(tuple_seq: &Vec<(T, T)>) -> Vec<T> {
//     // let mut result: Vec<T> = vec![];
//     let mut result;
//     for (i, row) in tuple_seq.iter().enumerate() {
//         if (i == 0) {
//             result = vec![row.0, row.1];
//         } else {
//             result.push(row.0);
//             result.push(row.1);
//         }
//     }

//     return result;
// }

// ¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯    \_ _ List Utils _ _/¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯ ¯    //

fn main() {
    println!("Hello, world!");
    let vec = vec![(1, 2), (3, 4), (5, 6)];
    println!("{:?}", vec);
    let x = flatten_tuple_sequence(&vec);
    println!("{:?}", x);
}
