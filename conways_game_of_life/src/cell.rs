use rand;
#[derive(Copy, Clone)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub alive: bool,
    alive_next_turn: bool,

}

impl Cell {

    pub fn new(x: u32, y: u32) ->  Cell {
        use rand::Rng;

        let num_gen = rand::thread_rng().gen_range(0, 5);
        let alive = match num_gen{
                1 => true,
                _ => false
            };
        Cell { x: x, y: y, alive: alive, alive_next_turn: false }
    }

    pub fn survive(&mut self, alive_neighbors:u32) -> () {
        self.alive_next_turn = match (self.alive, alive_neighbors) {
            (false, 3) => true,
            (true, 2) => true,
            (true, 3) => true,
            _ => false
        };
        ()
    }

    pub fn update(&mut self) -> () {
        self.alive = self.alive_next_turn;
        self.alive_next_turn = false;
        ()
    }
}
