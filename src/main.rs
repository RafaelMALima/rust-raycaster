use sdl2::{video::Window, render::Canvas, rect::Rect, EventPump};
mod event_handler; /*namespace declaration */ use event_handler::event_handler;
mod player; use player::Player;
mod map; use map::Level;

static SCREEN_WIDTH:u32 = 1280;
static SCREEN_HEIGHT:u32 = 720;


fn main() -> Result<(), String>{
    let context = sdl2::init()?;
    let video_subsystem = context.video()?;
    let window = video_subsystem.window("Raycaster", SCREEN_WIDTH, SCREEN_HEIGHT)
    .build()
    .unwrap();


    let level_1 = Level::new("../maps/test.map".to_string(), 0, "Test".to_string());
    
    let mut player = Player::new(vector2d::Vector2D { x: 1., y: 1. });

    let mut canvas:Canvas<Window> = window.into_canvas()
        .build()
        .unwrap();
    let _screen_area = Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);
    let clear_color = sdl2::pixels::Color::RGB(100,100,100);
    canvas.set_draw_color(clear_color);

    let mut running = true;
    let mut event_queue:EventPump = context.event_pump()
        .unwrap();

    //let keyboard_state_array = sdl2::keyboard::KeyboardStatmape::new(&event_queue);
    while running {
        let screen_area = Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);
        let _clear_color = sdl2::pixels::Color::RGB(100,100,100);
        running = event_handler(&mut event_queue);
        match canvas.fill_rect(screen_area){
            Ok(()) => {{}}
            Err(err_str) => println!("{}",err_str)
        }
        
        player.player_controller(&event_queue);
        match level_1.draw_map(&mut canvas, &player){
            Ok(()) => {  }
            Err(_err_str) => {  }
        }
        canvas.present();
    }
    Ok(())
}
