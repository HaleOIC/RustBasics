fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
    example_1_();
    example_2_();
}

fn identity(a: &i32) -> &i32 {
    a
}

// nothing will happen, it is correct
fn example_1_() {
    let x = 4;
    let x_ref = identity(&x);
    assert_eq!(*x_ref, 4);
}

/**
   not correct, x_ref lifetime is longer than
   the given variable x
*/

// Original version
// fn example_2() {
//     let mut x_ref: Option<&i32> = None;
//     {
//         let x = 7;
//         x_ref = Some(identity(&x));
//     }
//     assert_eq!(*x_ref.unwrap(), 7);
// }
// modified version
fn example_2_() {
    let mut x_ref: Option<&i32> = None;
    let x = 7;
    x_ref = Some(identity(&x));
    assert_eq!(*x_ref.unwrap(), 7);
}

// add some related lifetime parameter
fn option_or<'a>(opt: Option<&'a i32>, otherwise: &'a i32) -> &'a i32 {
    opt.unwrap_or(otherwise)
}

/* Nothing Else, it is correct. */
fn example_1() {
    let x = 8;
    let y = 10;
    let my_number = Some(&x);
    assert_eq!(&x, option_or(my_number, &y));
}

/* y is recycled but answer is its corresponding referrence. */
// Original version
// fn example_2() {
//     let answer = {
//         let y = 4;
//         option_or(None, &y)
//     };
//     assert_eq!(answer, &4);
// }
// Modified version
fn example_2() {
    let y: i32 = 4;
    let answer = { option_or(None, &y) };
    assert_eq!(answer, &4);
}

// No problem, it is correct.
fn example_3() {
    let y = 4;
    let answer = { option_or(None, &y) };
    assert_eq!(answer, &4);
}

/* x and y does not have the same lifetime */
// Original version
// fn example_4() {
//     let y = 4;
//     let answer = {
//         let x = 7;
//         option_or(Some(&x), &y)
//     };
//     assert_eq!(answer, &7);
// }
// Modified version
fn example_4() {
    let y = 4;
    let x = 7;
    let answer = { option_or(Some(&x), &y) };
    assert_eq!(answer, &7);
}
