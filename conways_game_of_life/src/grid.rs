use cell;

pub struct Grid {
    pub grid: Vec<Vec<cell::Cell>>,
    pub height: u32,
    pub width: u32
}

impl Grid {

    pub fn new(width: u32, height:u32) -> Grid {
        Grid {
            width: width,
            height: height,
            grid: {
                let mut temp_vec: Vec<Vec<cell::Cell>> = Vec::new();
                for i in  0..height {
                    temp_vec.push(Vec::new());
                    for j in 0..width {
                        temp_vec.get_mut(i as usize).unwrap().push(cell::Cell::new(j, i));
                    }
                }
                temp_vec
            }
        }
    }

    pub fn alive_cells(&self) -> Vec<cell::Cell>{
        let mut alive_cells: Vec<cell::Cell> = Vec::new();
        for cells in self.grid.iter() {
             for cell in cells.iter().filter(|&c| { c.alive }) {
                alive_cells.push(cell.clone());
            };
        }
        alive_cells
    }

    // TODO make less verbose ; perhaps use slices
    fn get_alive_neighbors(&self, x: u32, y: u32) -> u32 {
        let mut alive_neighbors= 0;
        if y != 0 {
            // top
            if self.grid.get((y - 1) as usize).unwrap().get(x as usize).unwrap().alive { alive_neighbors+=1; }
            if x != 0 {
                // top left
                if self.grid.get((y - 1) as usize).unwrap().get((x - 1) as usize).unwrap().alive  { alive_neighbors+=1; }
            }
            if x != self.width - 1 {
                // top right
                if self.grid.get((y - 1) as usize).unwrap().get((x + 1) as usize).unwrap().alive  { alive_neighbors+=1; }
            }
        }
        if x != 0 {
            // left
            if self.grid.get(y as usize).unwrap().get((x - 1) as usize).unwrap().alive  { alive_neighbors+=1; }
        }
        if x != self.width -1 {
            // right
            if self.grid.get(y as usize).unwrap().get((x + 1) as usize).unwrap().alive  { alive_neighbors+=1; }
        }
        if y != self.height - 1 {
            //bottom
            if self.grid.get((y + 1) as usize).unwrap().get(x as usize).unwrap().alive  { alive_neighbors+=1; }
            if x != 0 {
                // bottom left
                if self.grid.get((y + 1) as usize).unwrap().get((x - 1) as usize).unwrap().alive  { alive_neighbors+=1; }
            }
            if x != self.width - 1 {
                // bottom right
                if self.grid.get((y + 1) as usize).unwrap().get((x + 1) as usize).unwrap().alive  { alive_neighbors+=1; }
            }
        }
        alive_neighbors
    }

    pub fn tick(&mut self) -> () {
        // determine if cells will live or die
        for i in 0..self.height {
            for j in 0..self.width {
                let alive_neighbors = self.get_alive_neighbors(j, i);
                let mut cell = self.grid.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap();
                        cell.survive(alive_neighbors);
            }
        }
        // move to next turn
        for v in self.grid.iter_mut() {
            for cell in v.iter_mut() {
                (*cell).update();
            }
        }
        ()
    }
}
