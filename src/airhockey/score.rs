use crate::alloc;
alloc::vec::Vec;

pub struct Score {
    player_scores:Vec<u16>,
    max_score:u16
}
impl Score {
    // game constructor
    pub fn new(number_players: u8, max_score: u16) -> Game {
        let mut player_scores: Vec<u16> = Vec::new();
        for p in 0..number_players {
            players.push(0);
        }
        Game {
            player_scores: player_scores,
            max_score: max_score,
        }

    }
    
    // return 0 if no player has won else player number
    pub fn is_game_over(&self) {
        let mut i:u16 = 1;
        for p in self.player_scores {
            if (p >= self.max_score) 
            {
                return p;
            } 
            i += 1;
        }
        return 0;
    }

}
