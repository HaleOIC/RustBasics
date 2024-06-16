#[derive(PartialEq, Eq, Debug)]
pub enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    // TODO - implement map
}

#[derive(PartialEq, Eq, Debug)]
pub struct MyVec {
    items: Vec<i32>,
}

impl MyVec {
    // TODO - implement for_each
    // TODO - implement map
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    for arg in args {
        match arg.parse::<i32>() {
            Ok(1) => {
                test_option_map();
            }
            Ok(2) => {
                test_vec_map();
            }
            Ok(3) => {
                test_vec_for_each();
            }
            _ => {
                println!("Invalid test case");
            }
        }
    }
}

fn test_option_map() {
    let a = MyOption::Some(1).map(|x| x + 1);
    println!("{:?}", a);
}

fn test_vec_map() {
    let mut b = MyVec {
        items: vec![1, 2, 3],
    };

    let mut a = 1;
    b.map(|x| {
        a += 1;
        x * a
    });
    println!("{:?}", b);
}

fn test_vec_for_each() {
    let b = MyVec {
        items: vec![1, 2, 3],
    };

    b.for_each(|x| println!("{:?}", x));
}
