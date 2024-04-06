fn main() {
    let mut count = 0;
    loop {
        if count >= 10 {
            break;
        }
        print!("{}\n", count);
        count = count + 1;
    }
}
