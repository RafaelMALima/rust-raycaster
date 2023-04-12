use sdl2::{rect::Rect, video::Window};
use vector2d::Vector2D;
use crate::Player;
use std::fs;
use std::f64::consts::PI;


struct Sector{
    number:u16,
    points_vec:Vec<Vector2D<f64>>,
    floor_heigth:u16,
    ceiling_heigth:u16,
    portals:[u16;255],
}
pub struct Level{
    sectors:Vec<Sector>,
    pub id: u8,
    pub name:String,
}

impl Level{
    fn load_sectors(&mut self, map_path:String) -> Result<u16,String>{
        //let contents = fs::read_to_string(map_path);
        //for row in contents{
            //if row == "[SECTORS]"
            //começa a logica para setores
        //}
        //cria um setor dummy pra testar
        let setor = Sector{
            number:1,
            points_vec:vec!(
                Vector2D::new(1.,0.),
                Vector2D::new(2.,1.),
                Vector2D::new(1.,2.),
                Vector2D::new(0.,1.),
            ),
            floor_heigth:300,
            ceiling_heigth:0,
            portals:[0;255],
        };
        let sec_id = setor.number;
        self.sectors.push(setor);
        return Ok(sec_id);
    }   
    pub fn new(map_path:String, levl_id:u8, levl_name:String)-> Self{
        let mut lvl = Level{
            sectors:Vec::new(),
            id:levl_id,
            name:levl_name,
        };
        match lvl.load_sectors(map_path) {
            Ok(sector) => { println!("Sector {} loaded sucesfully",sector) },
            Err(error) => { println!("Failed to load sector, exited with error: {}",error) }
        }
        return lvl;
    }
    fn draw_sector(&self, player:&Player, fov:f64, sector: &Sector,screen:&mut sdl2::render::Canvas<Window>){
        for line in 0..player.fov{
            for i in 0..self.sectors.get(0).unwrap().points_vec.len(){
                let angle = (player.alpha - fov/2.) + (fov/(player.fov as f64))*line as f64;
                let point:Vector2D<f64> = player.pos + Vector2D { x: f64::cos(angle), y: f64::sin(angle) };
                let line_seg_start: &Vector2D<f64>;
                if i == 0{
                    line_seg_start = sector.points_vec.get(sector.points_vec.len()-1).unwrap();
                }
                else{
                    line_seg_start = sector.points_vec.get(i-1).unwrap();
                }
                let line_seg_end: &Vector2D<f64> = sector.points_vec.get(i).unwrap();
                let distance:Option<f64> = player.calculate_distance(point, line_seg_start, line_seg_end);
                match distance{
                    Some(dist) => {
                        //println!("{}, {}", i, dist);
                        match screen.fill_rect(Rect::new(line as i32,720/2 - (300./((dist+0.1))) as i32/2,1,(300./(dist+0.1)) as u32)){
                            Ok(()) => { }
                            Err(str) => {println!("{}",str)}
                        }
                        
                    }
                    _ => { /*do something*/}
                }
            }
        }
    }
    pub fn draw_map(&self,screen:&mut sdl2::render::Canvas<Window>,player:&Player) -> Result<(), u8>{ 
        let wall_color = sdl2::pixels::Color::RGB(255,255,255);
        screen.set_draw_color(wall_color);
        let sector: &Sector = self.sectors.get(player.get_current_sector()).unwrap();
        self.draw_sector(player, PI/4., sector, screen);
        screen.set_draw_color(sdl2::pixels::Color::RGB(100,100,100));// clear color 
        Ok(())
    }
}
