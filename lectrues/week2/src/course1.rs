// Monday Topic: Array, Vec, VecDeque, String, LinkedList
// HashMap BTreeMap HashSet BTreeSet BinaryHeap

use std::collections::HashMap;


// 1. Array will restrict its size as its initialization
// 2. Vec is dynamic, which means it can remove, add elements into it.
// 3. Iterator, in rust it will use Iterator.next() -> Optional<type of Item>
// 4. Map 
pub fn main() {
    println!("Week2 Monday course");
    vec_part();
    iter_part();
    map_part();
}   

fn vec_part() {
    let mut my_vec = vec![1, 2, 3];
    
    // make sure the vec is mutable
    my_vec.push(4);
    my_vec.push(5);
    my_vec.remove(2);

    for ele in my_vec.clone() {
        println!("element is {ele}");
    }
    println!("sum of list: {}", sum_list(my_vec.clone()));
    let double_vec = double_numbers(my_vec.clone());
    println!("{double_vec:?}");
}


fn sum_list(array: Vec<i32>) -> i32 {
    let mut sum = 0;
    for elem in array {
        sum += elem;
    }
    sum
}


fn double_numbers(vec: Vec<i32>) -> Vec<i32> {
    // just one time allocation and assign values 
    // no reallocation operation happens 
    let mut double_vec = Vec::with_capacity(vec.len());
    println!("{}", double_vec.capacity());
    for elem in vec {
        double_vec.push(elem * 2);
    }
    double_vec
}


fn iter_part() {
    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    let average = mean_imperative(my_vec.clone())
        .expect("Vector is not empty, see line above");
    let avg = mean_function(my_vec.clone())
        .expect("Vector is not empty, see line above");
    println!("Average is {average}, Avg is {avg}");

    let mut iter = my_vec.into_iter()
        .map(|num| format!("{num}"));
//     println!("iter.next() is {:?}", iter.next());
//     println!("iter.next() is {:?}", iter.next());
//     println!("iter.next() is {:?}", iter.next());
//     println!("iter.next() is {:?}", iter.next());
//     // None.next is None
//     println!("iter.next() is {:?}", iter.next());
    loop {
        let item = iter.next();
        match item {
            Some(elem) => {
                println!("{elem}");
            }
            None => {
                break;
            }
        }
    }

    let x = vec![1, 2, 3, 4, 5, 6, 0, 8, 9];
    let y = vec![1, 10, 3, 4, 5, 6, 0, 8, 9];
    println!("imperical: {}, functional: {}", 
                longest_equal_run_imp(x.clone(), y.clone()),
                longest_equal_run_fun(x.clone(), y.clone()));

}

// This function returns `None` iff `x` is empty
fn mean_imperative(x: Vec<i32>) -> Option<f64> {
    if x.is_empty() {
        None
    } else { 
        let length = x.len();
        let mut overall_sum: i32 = 0;
        for elem in x {
            overall_sum += elem;
        }
        Some(overall_sum as f64 / length as f64)
    }
}

fn mean_function(x: Vec<i32>) -> Option<f64> {
    if x.is_empty() {
        None
    } else { 
        let length = x.len();
        let overall_sum: i32 = x.into_iter().sum();
        Some(overall_sum as f64 / length as f64)
    }
}

fn longest_equal_run_imp(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut longest_run = 0;
    let mut current_run = 0;
    
    let short_length = x.len().min(y.len());
    for index in 0..short_length {
        if x[index] == y[index] {
            current_run += 1;
            longest_run = current_run.max(longest_run);
        } else {
            current_run = 0;
        }
    }
    longest_run
} 

fn longest_equal_run_fun(x: Vec<i32>, y: Vec<i32>) -> usize {
    let (best, _) = x.into_iter()
        .zip(y.into_iter())
        .map(|(x_elem, y_elem)| x_elem == y_elem)
        .fold((0, 0), |(best, cur), elem| 
            if elem {
                let cur = cur + 1;
                (best.max(cur), cur)
            } else {
                (best, 0)
            }
        );
    best
}


fn map_part() {
    let z = vec![1, 1, 1, 2, 1, 5, 5, 5, 8, 5, 7, 5, 6, 5];
    println!("mode: {:?}", mode(z));
}

fn mode(vec: Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();
    
    for elem in vec {
        map.entry(elem)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let (num, _) = map.into_iter()
        .max_by_key(|(_, freq)| *freq)?;
    Some(num)
}