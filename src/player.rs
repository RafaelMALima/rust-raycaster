use sdl2::{rect::Rect, EventPump, keyboard::Scancode};

pub struct Player{
    x:f64,
    y:f64,
    dx:f64,
    dy:f64,
    alpha:f64,
}
impl Player{
    pub fn new()-> Self{
        return Player{ 
            x:0.0,
            y:0.0,
            dx:0.0,
            dy:0.0,
            alpha:0.0,
        }
    }
    pub fn check_collision(&self, cube:&Rect)->u8{
        if (self.x + self.dx > f64::from(cube.x) && self.x + self.dx < f64::from(cube.x) + f64::from(cube.w) ) // checa no eixo x
        && (self.y + self.dy > f64::from(cube.y) && self.y + self.dy < f64::from(cube.y) + f64::from(cube.h) ) //checa o eixo y
        {
            return 1; // 1 = colisao frontal
        }
        if (self.x - self.dx > f64::from(cube.x) && self.x - self.dx < f64::from(cube.x) + f64::from(cube.w) ) // checa no eixo x
        && (self.y - self.dy > f64::from(cube.y )&& self.y - self.dy < f64::from(cube.y) + f64::from(cube.h) ) //checa o eixo y
        { 
            return 2; // 2 = colisao traseira
        }
        return 0;
    }
    pub fn player_controller(&self, event_pump: &EventPump){
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::W){ println!("w pressed"); }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::A){ println!("a pressed"); }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::S){ println!("s pressed"); }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::D){ println!("d pressed"); }
        return;
    }
}