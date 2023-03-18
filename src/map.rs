use sdl2::{rect::Rect, render::Canvas, video::Window, pixels::Color};
use crate::Player;
pub struct Level{
    pub map_grid:[[u8;8];8],
    pub width:i32,
    pub heigth:i32,
    pub id: u8,
    pub name:String,
}
impl Level{
    pub fn new(grid:[[u8;8];8], cell_width:i32 , cell_heigth:i32 , levl_id:u8, levl_name:String)-> Self{
        Level{
            map_grid:grid,
            id:levl_id,
            name:levl_name,
            width:cell_width,
            heigth:cell_heigth,
        }
    }
    pub fn draw_map(&self,map_rect:&mut sdl2::render::Canvas<Window>, optional_Player:Option<&Player>){ //player must be wrapped inside Option enum eg.(Some(Player))
        let draw_color = sdl2::pixels::Color::RGB(10,10,10);
        for i in 0..8{
            for j in 0..8{
                if self.map_grid[i][j] == 1{
                    let my_rect = Rect::new(
                        self.width*(i as i32),
                        self.heigth*(j as i32),
                        self.width as u32,
                        self.heigth as u32,
                    );
                    let mut clear_color = sdl2::pixels::Color::RGB(10,10,10);
                    map_rect.set_draw_color(clear_color);
                    let result = map_rect.fill_rect(my_rect);
                    clear_color = sdl2::pixels::Color::RGB(100,100,100);
                    map_rect.set_draw_color(clear_color);
                }
            }
        }
        match optional_Player{
            Some(Player) => { }
            _ => {  }
        }
    }
}
