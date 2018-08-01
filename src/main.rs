extern crate sdl2;
extern crate rand;
extern crate life_game;

//use life_game::CellularAutomata;
use life_game::AutomataRenderer;
use life_game::life::LifeBoard;
use life_game::life::LifeCellState;





const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;
const BOARD_SIZE: usize = 150;
const FRAMES_PER_SECOND: u64 = 10;
const P_LIFE: f32 = 0.05;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Conway's Game of Life", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let board = LifeBoard::new_random(BOARD_SIZE, BOARD_SIZE, P_LIFE).unwrap();
    let mut renderer = AutomataRenderer::new(&sdl_context, &mut canvas, &texture_creator, board, FRAMES_PER_SECOND, life_color);
    let (frames, duration) = renderer.run();
    println!("Ran {} frames in {}.{} seconds", frames, duration.as_secs(), duration.subsec_millis());
    println!("That's {} frames per second.", frames as f32 / (duration.as_secs() as f32 + duration.subsec_millis() as f32 / 1000.0))
}

fn life_color(cell: LifeCellState) -> (u8, u8, u8) {
    match cell {
        LifeCellState::Alive => (0, 0, 0),
        LifeCellState::Dead => (255, 255, 255)
    }
}

