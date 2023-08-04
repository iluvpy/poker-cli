

pub struct Player {
    pub name: String,
    pub credit: i32,
}

impl Player {
    pub fn new(name: String, credit: i32) -> Player {
        return Player{name: name, credit: credit};
    }
}



