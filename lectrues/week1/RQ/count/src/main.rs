fn main() {
    let mut count = 0;

    // /* Original version */
    // loop {
    //     if count >= 10 {
    //         break;
    //     }
    //     print!("{}\n", count);
    //     count = count + 1;
    // }

    /* Fixed version */
    while count < 10 {
        print!("{}\n", count);
        count = count + 1;
    }

}
