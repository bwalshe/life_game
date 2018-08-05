use rand::{thread_rng, Rng};

use super::CellularAutomata;

pub struct BrainBoard{
    cells: Vec<BrainCellState>,
    width: usize,
    height: usize
}

#[derive(Clone)]
pub enum BrainCellState{
    Firing,
    Refactory,
    Dead
}

impl BrainBoard {
    /// Create a new `LifeBoard` with width `width`, and height `height`. Each of the cells 
    /// will be set to true with a probability specified by `p_live`
    pub fn new_random(width: usize, height: usize, p_live: f32) -> Result<BrainBoard, String> {
        if p_live < 0.0 || p_live > 1.0 {
            return Err("p_live must be between 0 and 1".into());
        }
        let ncells = width * height;
        let mut cells: Vec<BrainCellState> = Vec::with_capacity(ncells);
        let mut rng = thread_rng();
        for _ in 0..ncells{
            cells.push(if rng.gen_range(0.0f32, 1.0f32) < p_live {BrainCellState::Firing} 
                else {BrainCellState::Dead})
        }
        Ok(BrainBoard{cells, width, height})
    }

    /// Construct a new game board from an existing array
    /// * `width` must be non-zero
    /// * The length of `vals` must be a whole multiple of `width`
    pub fn from_array(vals: &[BrainCellState], width: usize) -> Result<BrainBoard, String>{
        if width == 0{
            return Err("With must be non-zero".into())
        }
        if vals.len() % width != 0 {
            return Err("Array length must be an integer multiple of width".into())
        }
        let height = vals.len() / width;
        let mut cells: Vec<BrainCellState> = Vec::with_capacity(vals.len());
        for cell in vals {
            cells.push(cell.clone());
        }
        Ok(BrainBoard{cells, width, height})
    }
    
    fn count_neighbours(&self, i:i32, j:i32) -> u8 {
        self.get_neighbours(i, j).iter()
            .filter(|cell| match cell { 
                 BrainCellState::Firing => true,
                 _ => false
            })
            .count() as u8
    }
}

impl CellularAutomata<BrainCellState> for BrainBoard {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
    
    /// Get the value of the cell located at (i, j).
    /// Will return false for any i or j are negative or greater than the width/height of the 
    /// board
    fn get_cell(&self, i:i32, j:i32) -> BrainCellState {
        if self.invalid_coords(i, j){
            return BrainCellState::Dead;
        }
         
        return self.cells[i as usize + j as usize * self.width].clone()
    }
     ///
    /// Advance the game forward one step
    /// 
    fn step(&mut self){
        let mut new_cells: Vec<BrainCellState> = Vec::with_capacity(self.cells.len());
        for j in 0..self.height as i32 {
            for i in 0..self.width as i32{
                match (self.get_cell(i,j), self.count_neighbours(i, j)){
                    (_, 2) =>  new_cells.push(BrainCellState::Firing),
                    (BrainCellState::Firing, _) => new_cells.push( BrainCellState::Refactory),
                    _ =>new_cells.push( BrainCellState::Dead)
                }
            }
        }
        self.cells = new_cells;
    }

}