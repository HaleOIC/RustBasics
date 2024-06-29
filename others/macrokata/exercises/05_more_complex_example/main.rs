////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! for_2d {
    ($var_x:ident <$type_x:ty> in $x_range:expr, $var_y:ident <$type_y:ty> in $y_range:expr, $body:block) => {
        for $var_x in $x_range {
            let $var_x: $type_x = $var_x;
            for $var_y in $y_range {
                let $var_y: $type_y = $var_y;
                $body
            }
        }
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        (Coordinate {x: col, y: row}).show()
    });

    let values = [1, 3, 5];

    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
