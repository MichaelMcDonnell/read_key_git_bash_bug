use console::Term;

fn main() {
    let term: Term = Term::buffered_stdout();

    println!("Enter any key:");
    let key = term.read_key();
    match key {
        Ok(key) => println!("You entered '{:?}'", key),
        Err(e) => eprintln!("Error, could not read key: {}", e),
    };
}
