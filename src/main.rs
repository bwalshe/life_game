extern crate sdl2;
extern crate rand;
extern crate life_game;

use life_game::LifeBoard;
use life_game::LifeCellState;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;
const BOARD_SIZE: usize = 150;
const FRAMES_PER_SECOND: u64 = 10;
const P_LIFE: f32 = 0.05;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, BOARD_SIZE as u32, BOARD_SIZE as u32).unwrap();
   
    let texture_info = texture.query();
    println!("{:?}", texture_info);
    
    let mut board = LifeBoard::new_random(BOARD_SIZE,BOARD_SIZE,0.05).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
   
    let start_time = std::time::Instant::now();
    let frame_duration = std::time::Duration::from_millis(1000/FRAMES_PER_SECOND);
    
    let mut frames = 0;
    
    canvas.clear();
    
    'running: loop {
         let frame_started = std::time::Instant::now();
         texture.with_lock(None, |buffer: &mut [u8], pitch: usize | {
            for y in 0..texture_info.height as usize {
                for x in 0..texture_info.width as usize {
                    let offset = y * pitch + x * 3;
                    let col = match board.get_cell(x as i32, y as i32) {
                        LifeCellState::Alive => 0,
                        LifeCellState::Dead => 255
                    };
                    buffer[offset] = col;
                    buffer[offset + 1] = col;
                    buffer[offset + 2] = col;
                }
            }
        }).unwrap();    
        
        canvas.copy(&texture, None, None).unwrap();           
        canvas.present();
        
        frames+=1;
        board.step();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
                    board = LifeBoard::new_random(BOARD_SIZE,BOARD_SIZE,P_LIFE).unwrap();
                }
                _ => {}
            }
        }
    
        let expired_time = std::time::Instant::now() - frame_started;    
        std::thread::sleep(frame_duration - std::cmp::min(frame_duration, expired_time));
        
    }

    let duration = std::time::Instant::now() - start_time;
    println!("{} frames displayed, in {}.{} seconds. That's {} frames per second", frames, duration.as_secs(), 
            duration.subsec_millis(), frames as f32 / (duration.as_secs() as f32 + duration.subsec_millis()  as f32 / 1000.0));
}

