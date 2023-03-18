use sdl2::{video::{self, Window}, render::Canvas, rect::Rect, event::Event, EventPump};
mod event_handler; /*namespace declaration */ use event_handler::event_handler;
mod player; use player::Player;
mod map; use map::Level;

static SCREEN_WIDTH:u32 = 800;
static SCREEN_HEIGHT:u32 = 600;

fn distance(x1:f64,y1:f64,x2:f64,y2:f64)-> f64{
    return f64::sqrt((x2 - x1)*(x2 - x1) +(y2 - y1)*(y2 - y1));
}

fn main() -> Result<(), String>{
    let context = sdl2::init()?;
    let video_subsystem = context.video()?;
    let window = video_subsystem.window("Raycaster", SCREEN_WIDTH, SCREEN_HEIGHT)
    .build()
    .unwrap();


    let level_1 = Level::new([
        [1,1,1,1,1,1,1,1],
        [1,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1],],
        (SCREEN_WIDTH/8) as i32,(SCREEN_HEIGHT/8) as i32,1,"Lvl1".to_string());
    
    let player = Player::new();

    let mut canvas:Canvas<Window> = window.into_canvas()
        .build()
        .unwrap();
    let screen_area = Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);
    let clear_color = sdl2::pixels::Color::RGB(100,100,100);
    canvas.set_draw_color(clear_color);

    let mut running = true;
    let mut event_queue:EventPump = context.event_pump()
        .unwrap();

    //let keyboard_state_array = sdl2::keyboard::KeyboardStatmape::new(&event_queue);
    let player1 = Player::new();
    while running {
        let screen_area = Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);
        let clear_color = sdl2::pixels::Color::RGB(100,100,100);
        running = event_handler(&mut event_queue);
        canvas.fill_rect(screen_area);
        
        player1.player_controller(&event_queue);
        level_1.draw_map(&mut canvas, Some(&player));
        canvas.present();
    }
    Ok(())
}