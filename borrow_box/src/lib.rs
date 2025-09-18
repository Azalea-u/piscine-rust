#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u32),
    pub p2: (String, u32),
    pub nb_games: u32,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u32) -> Self {
        Self {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        }
    }

    pub fn read_winner(&self) -> Option<&(String, u32)> {
        let p1_score = self.p1.1;
        let p2_score = self.p2.1;

        if p1_score > p2_score {
            Some(&self.p1)
        } else if p2_score > p1_score {
            Some(&self.p2)
        } else {
            None
        }
    }

    pub fn update_score(&mut self, user_name: &str) {
       if self.p1.1 >= self.nb_games / 2 + 1 || self.p2.1 >= self.nb_games /2 + 1 {
           return;
       }

        if user_name == self.p1.0 {
            self.p1.1 += 1;
        } else if user_name == self.p2.0 {
            self.p2.1 += 1;
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
