//! doctor_who
//! Crate used to perform caesar shifts

/// Default shift if no other shift specified
const DEFAULT_SHIFT: i32 = 5;

/// numeric ASCII value for 'A'
const UPPERCASE_A: i32 = 65;
/// numeric ASCII value for 'a'
const LOWERCASE_A: i32 = 97;
/// numeber of letters in the alphabet
const ALPHABET_SIZE: i32 = 26;

pub fn caesar_shift(shift_by: Option<i32>, lines: Vec<String>) -> Vec<String> {
    let shift_number = shift_by.unwrap_or(DEFAULT_SHIFT);

    // no idea what this is doing? Ask the forums and/or
    // look back at the functional programming lectures!
    lines
        .iter()
        .map(|line| shift(shift_number, line.to_string()))
        .collect()
}

/// Shift each letter in vec of lines, by shift amount
/// # Arguments
/// * `shift_by` - The amount to shift each letter by. If None, use DEFAULT_SHIFT.
/// * `lines` - Vec of lines to apply the function on.
///
/// # Examples
///
/// ```
/// let lines = vec![String::from("Hello, World!")];
/// let shifted = doctor_who::caesar_shift(Some(3), lines);
/// assert_eq!(shifted, vec![String::from("Khoor, Zruog!")]);
///
/// let lines = vec![String::from("Hello, World!")];
/// let shifted = doctor_who::caesar_shift(None, lines);
/// assert_eq!(shifted, vec![String::from("Mjqqt, Btwqi!")]);
/// ```
///
fn shift(shift_by: i32, line: String) -> String {
    let mut result: Vec<char> = Vec::new();

    // turn shift_by into a positive number between 0 and 25
    let shift_by = shift_by % ALPHABET_SIZE + ALPHABET_SIZE;

    line.chars().for_each(|c| {
        let ascii = c as i32;

        if ('A'..='Z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - UPPERCASE_A) + shift_by, ALPHABET_SIZE) + UPPERCASE_A,
            ));
        } else if ('a'..='z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - LOWERCASE_A) + shift_by, ALPHABET_SIZE) + LOWERCASE_A,
            ));
        } else {
            result.push(c)
        }
    });

    result.iter().collect()
}

fn abs_modulo(a: i32, b: i32) -> i32 {
    (a % b).abs()
}

fn to_ascii(i: i32) -> char {
    char::from_u32(i as u32).unwrap()
}
