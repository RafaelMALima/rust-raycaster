use sdl2::{rect::Rect, render::Canvas};
use player::Player;
pub struct Level{
    map_grid:[[u8;8];8],
    width:i32,
    heigth:i32,
    id: u8,
    name:String,
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
    pub fn draw_map(&self, map_rect:sdl2::render::Canvas<T>, optional_Player:Option<Player>){ //player must be wrapped inside Option enum eg.(Some(Player))
        let draw_color = sdl2::pixels::Color::RGB(10,10,10);
        for i in 0..7{
            for j in 0..7{
                if self.map_grid[i][j] == 1{
                    let my_rect:Rect;
                    my_rect.w = self.width;
                    my_rect.h = self.heigth;
                    my_rect.x = my_rect.w*(i as i32);
                    my_rect.y = my_rect.h*(j as i32);
                    map_rect.draw_rect(my_rect);
                }
            }
        }
        match optional_Player{
            Some(Player) => { }
            _ => {  }
        }
    }
}
