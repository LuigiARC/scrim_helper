use scrim_helper::Character;
use scrim_helper::Player;
use scrim_helper::Team;
use scrim_helper::PLACEMENT_POINTS;

pub fn manual(num_teams: u32){
    println!("Manual mode");
    println!("Enter the details for each team manually for each round");
    
    // create team collection of Team type
    let mut teams: Vec<Team> = Vec::new();
    
    // loop for each of the teams in num_teams 
    // create teams and players strcutures and push them to the teams vector
    
    for i in 0..num_teams {
        // Team name
        println!("Enter the name of team {}", i + 1);
        let mut team_name = String::new();
        std::io::stdin()
        .read_line(&mut team_name)
        .expect("Failed to read line");
        let team_name = team_name.trim().to_string();

        // Team identifier
        println!("Enter the identifier of team {}", i + 1);
        let mut team_identifier = String::new();
        std::io::stdin()
            .read_line(&mut team_identifier)
            .expect("Failed to read line");
        let team_identifier = team_identifier.trim().to_string();

        // Players
        let mut players = Vec::new();
        for j in 0..4 {
            println!("Enter the name of player {}", j + 1);
            let mut player_name = String::new();
            std::io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");
            let player_name = player_name.trim().to_string();
            
            println!("Enter the character of player {}", j + 1);
            let mut player_character = String::new();
            std::io::stdin()
            .read_line(&mut player_character)
            .expect("Failed to read line");
            let player_character = player_character.trim().to_string();
            
            let character = match player_character.as_str() {
                "Brall" => Character::Brall,
                "Ghost" => Character::Ghost,
                "Jin" => Character::Jin,
                "Joule" => Character::Joule,
                "Myth" => Character::Myth,
                "Shiv" => Character::Shiv,
                "Shrike" => Character::Shrike,
                "Bishop" => Character::Bishop,
                "Kingpin" => Character::Kingpin,
                "Felix" => Character::Felix,
                "Oath" => Character::Oath,
                "Elluna" => Character::Elluna,
                "Zeph" => Character::Zeph,
                "Celeste" => Character::Celeste,
                "Hudson" => Character::Hudson,
                "Void" => Character::Void,
                _ => {
                    println!("Invalid character. Please enter a valid character");
                    break;
                }
            };
            
            let player = Player {
                name: player_name,
                character: character,
                kills: 0,
                deaths: 0,
                revives: 0,
                assists: 0,
                score: 0,
            };
            
            players.push(player);
        }
        
        let team = Team {
            name: team_name,
            team_identifier: team_identifier,
            players: players,
            team_total_kills: 0,
            team_placement_points: Vec::new(),
            bonus_points: 0,
            team_total_points: 0,
        };
        teams.push(team);
    }
    
    // ask user how many game will be played and if there are bonus games
    println!("How many games will be played?");
    let mut num_games = String::new();
    std::io::stdin()
        .read_line(&mut num_games)
        .expect("Failed to read line");
    
    println!("How many of those are bonus games?");
    let mut num_bonus_games = String::new();
    std::io::stdin()
        .read_line(&mut num_bonus_games)
        .expect("Failed to read line");
    
    
    let num_games: u32 = num_games.trim().parse::<u32>().expect("Please type a number!");
    let num_bonus_games: u32 = num_bonus_games.trim().parse::<u32>().expect("Please type a number!");
    let num_games = num_games - num_bonus_games;
    
    // Request placement for bonus games
    // loop for bonus games length and ask for the placement for top 3 teams
    let mut bonus_points = Vec::new();
    for i in 0..num_bonus_games {
        println!("Which team ended first in bonus game {}", i + 1);
        let mut first_place = String::new();
        std::io::stdin()
            .read_line(&mut first_place)
            .expect("Failed to read line");
        let first_place = first_place.trim().to_string();

        println!("Which team ended second in bonus game {}", i + 1);
        let mut second_place = String::new();
        std::io::stdin()
            .read_line(&mut second_place)
            .expect("Failed to read line");
        let second_place = second_place.trim().to_string();

        println!("Which team ended third in bonus game {}", i + 1);
        let mut third_place = String::new();
        std::io::stdin()
            .read_line(&mut third_place)
            .expect("Failed to read line");
        let third_place = third_place.trim().to_string();

        bonus_points.push((first_place.clone(), second_place.clone(), third_place.clone()));
        for (_index, team) in teams.iter_mut().enumerate() {
                if team.team_identifier == first_place {
                    team.bonus_points += 15;
                } else if team.team_identifier == second_place {
                    team.bonus_points += 10;
                } else if team.team_identifier == third_place {
                    team.bonus_points += 5;
                }
            }
        }

        // Print the teams and their bonus points
        for team in &teams {
        println!("Team {}: {} bonus points", team.name, team.bonus_points);
        }

        // loop for each game and ask for the total team kills and team placement 
        // for each team in each game. display options after each game input
        // and allow user to choose between entering the next game or finish

        for i in 0..num_games {
            println!("Game {}", i + 1);
            // loop for each team and ask for the total kills and placement
            for (_index, team) in teams.iter_mut().enumerate() {
                println!("Enter the total kills for team {}", team.name);
                let mut total_kills = String::new();
                std::io::stdin()
                    .read_line(&mut total_kills)
                    .expect("Failed to read line");
                let total_kills: u32 = total_kills.trim().parse().expect("Please type a number!");
                team.team_total_kills += total_kills;

                println!("Enter the placement for team {}", team.name);
                let mut placement = String::new();
                std::io::stdin()
                    .read_line(&mut placement)
                    .expect("Failed to read line");
                let placement: u32 = placement.trim().parse().expect("Please type a number!");
                
                // Find points for this placement from PLACEMENT_POINTS
                let points = PLACEMENT_POINTS
                    .iter()
                    .find(|(place, _)| *place == placement)
                    .map(|(_, points)| *points)
                    .expect("Invalid placement");
                
                // Add points to team's placement points vector
                team.team_placement_points.push(points);

            }
            // sort the teams by total kills
            teams.sort_by(|a, b| b.team_total_kills.cmp(&a.team_total_kills));
            // tally points up. kills are worth 1 point each, placement points and bonus points are added
            for (_index, team) in teams.iter_mut().enumerate() {
                team.team_total_points = team.team_total_kills + team.team_placement_points.iter().sum::<u32>() + team.bonus_points;
            } 

            // Print the teams and their total points
            for team in &teams {
                println!("Team {}: {} total points", team.name, team.team_total_points);
            }

            // TODO: Parse Stats and print in oraganized and easily understood fashion using calc_stats function


            // ask user if they want to enter the next game or finish
            println!("Enter 'n' to enter the next game or 'f' to finish");
            let mut choice = String::new();
            std::io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
            match choice.trim() {
                "n" => continue,
                "f" => break,
                _ => {
                    println!("Invalid choice. Please enter 'n' or 'f'");
                }
            }
        }

}