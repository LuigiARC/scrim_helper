use std::io;
mod modes;

fn main() {
    println!("Scrim Helper");

    // Let user choose between enter the details for each team manually for each round
    // or let the program generate the details for each team automatically from screenshots
    println!("How many teams are there?");
    let mut num_teams = String::new();

    io::stdin()
        .read_line(&mut num_teams)
        .expect("Failed to read line");

    let num_teams: u32 = num_teams.trim().parse().expect("Please type a number!");

    println!("(m)anual or (a)utomatic");
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "m" => {
                modes::manual::manual(num_teams);
                break;
            }
            "a" => {
                modes::automatic::automatic(num_teams);
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 'm' or 'a'");
            }
        }
    }
    
}
