pub enum Character {
    Brall,
    Ghost,
    Jin,
    Joule,
    Myth,
    Shiv,
    Shrike,
    Bishop,
    Kingpin,
    Felix,
    Oath,
    Elluna,
    Zeph,
    Celeste,
    Hudson,
    Void
}

pub const PLACEMENT_POINTS: [(u32, u32); 12] = [
    (1, 15), 
    (2, 12),  
    (3, 10),  
    (4, 8),   
    (5, 7),   
    (6, 6),   
    (7, 5),   
    (8, 4),   
    (9, 3),   
    (10, 2),  
    (11, 1),  
    (12, 1),  
];

// ...existing code...

pub struct Player {
    pub name: String,
    pub character: Character,
    pub kills: u32,
    pub deaths: u32,
    pub revives: u32,
    pub assists: u32,
    pub score: u32,
}

pub struct Team {
    pub name: String,
    pub team_identifier: String,
    pub players: Vec<Player>,
    pub team_total_kills: u32,
    pub team_placement_points: Vec<u32>,
    pub bonus_points: u32,
    pub team_total_points: u32
}