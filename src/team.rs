#[derive(Debug)]
pub struct Team {
    name: String,
    players: Vec<(i32, String, usize)>,  // (id, name, zone)
    coach: String,
    //TODO: implement special operations
    //timeouts: usize,
    //challenges: usize,
    //changes: usize,
    //changed_players: Vec<(i32, i32)>
}

impl Team {
    pub fn new() -> Self {
        Team {
            name: String::new(),
            players: Vec::new(),
            coach: String::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_coach(&mut self, name: String) {
        self.coach = name;
    }

    pub fn add_player(&mut self, player_id: i32, name: String, zone: usize) {
        // player's zone could be 0-6, where 0 is for MB/Libero off the court
        self.players.push((player_id, name, zone));
    }

    pub fn change_player(&mut self, off_player_id: i32, on_player_id: i32) {
        // change players

    }

    pub fn rotate(&mut self) {
        // moves players on the court counter clockwise
    }
}
