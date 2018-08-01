extern crate sdl2;
extern crate rand;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

pub mod life;

///
/// Generic 2D grid Celular Automata where each state is in a state of
/// type T
/// It has
///  * `step()` sets all cells on the board  to their next state
///  * `get_cell(i, j)` gets the state of the cell in the i,j th position
///  * functions to get the width and height of the board
/// 
pub trait CellularAutomata<T>{
    fn step(&mut self);
    fn get_cell(&self,  i:i32, j:i32) -> T;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

///
/// An object which will render a CelularAutomata to an SDL2 surface
/// In addtition to the automata and the SDL2 details, it needs
/// a function `f(T)->(u8, u8, u8) which converts the state of a cell to an rgb
/// triple
/// 
/// Once the renderer has been constructed, use `run()` to get it to 
/// loop infinately calling `board.step()` and displaying the result.
pub struct AutomataRenderer<'a, T, U:CellularAutomata<T>>{
    board: U,
    canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: sdl2::render::Texture<'a>,
    frame_duration: std::time::Duration,
    cell_to_rgb: fn(T) -> (u8, u8, u8),
    event_pump: sdl2::EventPump
}


impl <'a, T, U:CellularAutomata<T>> AutomataRenderer<'a, T, U>{
    pub fn new(
        sdl_context: &sdl2::Sdl,
        canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        board: U,
        fps:u64,
        cell_to_rgb: fn(T) -> (u8, u8, u8)) ->  Result<Self, Box<std::error::Error>> {
    
        

        let texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24, board.get_width() as u32, board.get_height() as u32)?;
    
        let texture_info = texture.query();
        println!("{:?}", texture_info);

        let frame_duration =  std::time::Duration::from_millis(1000/fps);
        let event_pump = sdl_context.event_pump()?;

        Ok(AutomataRenderer{board:board, 
                         canvas: canvas, 
                         texture: texture, 
                         frame_duration: frame_duration, 
                         cell_to_rgb: cell_to_rgb, 
                         event_pump: event_pump})
    }


    pub fn run(&mut self) -> Result<(i32, std::time::Duration), Box<std::error::Error>>  {
        let mut frames = 0;
        let game_started = std::time::Instant::now();
        
        'running: loop {
            let frame_started = std::time::Instant::now();
            let height = self.board.get_height() as usize;
            let width = self.board.get_width() as usize;
            {
                let texture = &mut self.texture;
                let board = &self.board;
                let col_fn = &self.cell_to_rgb;
                texture.with_lock(None, |buffer: &mut [u8], pitch: usize | {
                    for y in 0..height as usize {
                        for x in 0..width as usize {
                            let offset = y * pitch + x * 3;
                            let cell = board.get_cell(x as i32, y as i32);
                            let (r, g, b) = (col_fn)(cell);
                            buffer[offset] = r;
                            buffer[offset + 1] = g;
                            buffer[offset + 2] = b;
                            
                        }
                    }
                })?;
            } 
            self.canvas.copy(&self.texture, None, None)?; 
            self.canvas.present();
            frames+=1;
            self.board.step();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} 
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
                        //TODO: Spawn a new automata here. Will probably require 
                        //passing a function in the constructor which constructs 
                        //new automata
                    },
                    _ => {}
                }
            }
    
            let expired_time = std::time::Instant::now() - frame_started;    
            std::thread::sleep(self.frame_duration - std::cmp::min(self.frame_duration, expired_time));
        }
        
        Ok((frames, std::time::Instant::now() - game_started))
    }
}

