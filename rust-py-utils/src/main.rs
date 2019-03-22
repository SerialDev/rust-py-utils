
use std::collections::HashMap;

pub fn  uniquify_list(data:&Vec<i64>)-> Vec<i64>{
    let mut seen: HashMap<i64, i64> = HashMap::new();
    let mut result:Vec<i64> = Vec::new();
    for item in data.iter(){
        if !seen.contains_key(item){
            seen.insert(*item, *item);
            result.push(*item);
        }
        println!("{}", item);}
}
    return result;




fn main() {
    println!("Hello, world!");
}
