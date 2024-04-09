// Wednesday 

// Today topic: error handling 
// std::panic::unwind

pub fn main() {
    let num = helper1();
    println!("the double of input is {}", num * 2);
}


enum GetInputError {
    InputFailed,
    InputEmpty,
    InputWasNotNumber {
        input: String
    },
    // ...
}


fn get_input() -> Result<i32, GetInputError> {
    let mut input = String::new();
    let result = std::io::stdin()
        .read_line(&mut input);

    match result {
        Ok(_) => {},
        Err(_) => {
            return Err(GetInputError::InputFailed);
        }
    }

    if input.is_empty() {
        return Err(GetInputError::InputEmpty);
    }

    let num:Result<i32, _> = input.trim().parse();
    match num {
        Ok(value) => Ok(value),
        Err(_) => Err(GetInputError::InputWasNotNumber {
            input,
        })
    }

}


fn helper1() -> i32 {
    loop {
        match get_input() {
            Ok(value) => {
                return value;
            },
            Err(err) => {
                match err {
                    GetInputError::InputFailed | GetInputError::InputEmpty=> {
                        panic!("Could not read from stdin");
                    }
                    GetInputError::InputWasNotNumber { input } => {
                        println!("That was not a number!");
                    }
                }
            }
        }
    }
}