extern crate rand;
use rand::{thread_rng, Rng};

///
/// A game-board for the game of life
/// 
pub struct LifeBoard{
    cells: Vec<u8>,
    width: usize,
    height: usize
}

pub enum LifeCellState{
    Alive,
    Dead
}

impl LifeBoard {

    /// Create a new `LifeBoard` with width `width`, and height `height`. Each of the cells 
    /// will be set to true with a probability specified by `p_live`
    pub fn new_random(width: usize, height: usize, p_live: f32) -> Result<LifeBoard,String> {
        if p_live < 0.0 || p_live > 1.0 {
            return Err("p_live must be between 0 and 1".into());
        }
        let ncells = width * height;
        let mut cells: Vec<u8> = Vec::with_capacity(ncells);
        let mut rng = thread_rng();
        for _ in 0..ncells{
            cells.push(if rng.gen_range(0.0f32, 1.0f32) < p_live {1} else {0})
        }
        Ok(LifeBoard{cells, width, height})
    }

    /// Construct a new game board from an existing array
    /// * `width` must be non-zero
    /// * The length of `vals` must be a whole multiple of `width`
    pub fn from_array(vals: &[u8], width: usize) -> Result<LifeBoard,String>{
        if width == 0{
            return Err("With must be non-zero".into())
        }
        if vals.len() % width != 0 {
            return Err("Array length must be an integer multiple of width".into())
        }
        let height = vals.len() / width;
        let cells = vals.to_vec();
        Ok(LifeBoard{cells, width, height})
    }

    /// Get the value of the cell located at (i, j).
    /// Will return false for any i or j are negative or greater than the width/height of the 
    /// board
    pub fn get_cell(&self, i:i32, j:i32) -> LifeCellState {
        if self.invalid_coords(i, j){
            return LifeCellState::Dead;
        }
        if self.cells[i as usize + j as usize * self.width] == 0 {
            LifeCellState::Dead
        } else {
            LifeCellState::Alive
        }
    }

    fn set_cell(&mut self, i:i32, j:i32, value: LifeCellState){
        if !self.invalid_coords(i, j){
            self.cells[i as usize + j as usize *self.width] = match value {
                LifeCellState::Alive => 1,
                LifeCellState::Dead => 0
            }
        }
    }

    ///
    /// Advance the game forward one step
    /// 
    pub fn step(&mut self){
        for i in 0..self.width as i32{
            for j in 0..self.height as i32 {
                match (self.get_cell(i,j), self.count_neighbours(i, j)){
                    (LifeCellState::Alive, 2) 
                    | (LifeCellState::Alive, 3)  
                    | (LifeCellState::Dead, 3) => self.set_cell(i, j, LifeCellState::Alive),
                    _ => self.set_cell(i, j, LifeCellState::Dead)
                }
            }
        }
    }

    fn invalid_coords(&self, i:i32, j:i32) -> bool {
        i<0 || j<0 || i as usize >=self.width || j as usize >= self.height
    }

    fn count_neighbours(&self, i:i32, j:i32) -> u8 {
        let neighbours = [(i-1,j-1), (i,j-1), (i+1, j-1),
                          (i-1,j), (i+1,j),
                          (i-1, j+1), (i,j+1), (i+1, j+1)];
        
        neighbours.iter()
            .filter(|(x,y)| match self.get_cell(*x,*y) { 
                 LifeCellState::Alive => true,
                 LifeCellState::Dead => false
            })
            .count() as u8
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbours(){
        let game =  LifeBoard::from_array(&[1,1,1,1,1,1,1,1,1], 3).unwrap();
        
        assert_eq!(3, game.count_neighbours(0, 0));
        assert_eq!(5, game.count_neighbours(1, 0));
        assert_eq!(5, game.count_neighbours(0, 1));
        assert_eq!(8, game.count_neighbours(1, 1));

    }
}