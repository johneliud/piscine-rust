#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }

    pub fn read_winner(&self) -> (String, u16) {
        let (name1, score1) = &self.p1;
        let (name2, score2) = &self.p2;

        if score1 == score2 {
            ("Same score! tied".to_string(), *score1)
        } else if score1 > score2 {
            (name1.clone(), *score1)
        } else {
            (name2.clone(), *score2)
        }
    }
}
