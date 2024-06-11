/**
 * Reason for this program can not compile:
 *  first of all, declare vec as a mutable variable, and then make a reference of it
 *  by using new notation a, but it can not be borrowed twice using another new notation
 *  b, which is illegal.
 *
 * Fix:
 *  Just exchange the order of commands, this will tell compiler that `a` has finished
 *  his work and can be borrowed to `b`.
 */
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let a = &mut vec;
    a.push(11);
    let b = &mut vec;
    b.push(12);
}
