// hmm this doesn't look right!!
struct UniverseDetails {
    universe_name: String,
    universe_winner: String,
    universe_population: u32,
}

fn get_universe_details(universe_id: u32) -> Option<UniverseDetails> {
    // does this even compile??
    let rv = match universe_id % 15 {
        0 => Some(UniverseDetails {
                universe_name: String::from("Stardew Valley"),
                universe_winner: String::from("Jojo Corp"),
                universe_population: 1,
            }),
        3 | 6 | 9 | 12 => Some(UniverseDetails {
            universe_name: String::from("Star Wars"),
            universe_winner: String::from("The Rebellion"),
            universe_population: 4294967295
        }),
        5 | 10 => Some(UniverseDetails {
            universe_name: String::from("Miraculous"),
            universe_winner: String::from("Hawk Moth"),
            universe_population: 22,
        }),
        _ => None
    };
    return rv;
}


// this main function is fine, except for two gaps
// the print statements could make use of "{variable}" instead of 
// ("{}", variable)
fn main() {
    for id in 1..=15 {
        let universe_details = get_universe_details(id);
        if let Some(details) = universe_details {
            println!("Universe with id {} is called {}, won by {} and has a population of {}", 
                id, 
                details.universe_name, 
                details.universe_winner, 
                details.universe_population
            );
        } else {
            println!("Universe with id {} is unknown", id);
        }
    }
}
