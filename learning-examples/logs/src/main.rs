use std::fs;
use std::io::Error;

fn get_errors(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with("ERROR"))
        .map(|line| line.to_string()) // makes a copy in memory
        .collect()
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let errors = get_errors(&text);
    fs::write("errors.txt", errors.join("\n"))?;

    Ok(())

    // let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
    // let errors = get_errors(&text);
    // fs::write("errors.txt", errors.join("\n")).expect("Failed to write errors.txt");

    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let errors = get_errors(&text);
    //         match fs::write("errors.txt", errors.join("\n")) {
    //             Ok(()) => println!("Errors written to errors.txt"),
    //             Err(e) => println!("Failed to write errors.txt: {}", e),
    //         }
    //     }
    //     Err(e) => eprintln!("Failed to read logs.txt: {}", e),
    // }

   
}
