use std::collections::HashMap;
const ENROLMENTS_PATH: &str = "enrolments.psv";

fn main() {
    // Read into data file as reader 
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(ENROLMENTS_PATH)
        .expect("Failed to open csv file");

    // Init variables
    let mut students = HashMap::new();
    let mut courses = HashMap::new();

    // Parse result piece by piece
    for result in reader.records() {
        let record = result.expect("A CSV record error occurs");

        // wam at 5-th place
        let wam: f64 = record[5].to_string()
            .parse()
            .expect("Failed to parse wam");

        // add students into hashmap
        students.entry(record[1].to_string().clone())
            .and_modify(|x| *x = wam)
            .or_insert(wam);

        // cource code at the first place
        let course_code = record[0].to_string();
        courses.entry(course_code)
            .and_modify(|x| *x += 1)
            .or_insert(1); 
    }

    // fetch useful information
    let number_students = students.iter().count();
    let most_common_course = courses.iter()
        .max_by_key(|&(_, students)| students)
        .unwrap();
    let least_common_course = courses.iter()
        .min_by_key(|&(_, students)| students)
        .unwrap();
    let avg_wam = students.iter().map(|(_, wam)| wam).sum::<f64>() as f64 
        / students.len() as f64;


    println!("Number of students: {}", number_students);
    println!("Most common course: {} with {} students", most_common_course.0, most_common_course.1); 
    println!("Least common course: {} with {} students", least_common_course.0, least_common_course.1);
    println!("Average WAM: {:.2}", avg_wam);

}
