use sdl2::{rect::Rect, EventPump, keyboard::Scancode};
use vector2d::Vector2D;

pub struct Player{
    pub pos:Vector2D<f64>,
    pub speed:Vector2D<f64>,
    pub alpha:f64,
    pub fov:u16,
}
impl Player{
    pub fn new(pos: Vector2D<f64>)-> Self{
        return Player{ 
            pos:pos,
            speed:Vector2D { x: (0.), y: (0.) },
            alpha:0.0,
            fov:1280,
        }
    }
    pub fn _check_collision(&self, cube:&Rect)->u8{
        if (self.pos.x + self.speed.x > f64::from(cube.x) && self.pos.x + self.speed.x < f64::from(cube.x) + f64::from(cube.w) ) // checa no eixo x
        && (self.pos.y + self.speed.y > f64::from(cube.y) && self.pos.y + self.speed.y < f64::from(cube.y) + f64::from(cube.h) ) //checa o eixo y
        {
            return 1; // 1 = colisao frontal
        }
        if (self.pos.x - self.speed.x > f64::from(cube.x) && self.pos.x - self.speed.x < f64::from(cube.x) + f64::from(cube.w) ) // checa no eixo x
        && (self.pos.y - self.speed.y > f64::from(cube.y )&& self.pos.y - self.speed.y < f64::from(cube.y) + f64::from(cube.h) ) //checa o eixo y
        { 
            return 2; // 2 = colisao traseira
        }
        return 0;
    }
    pub fn player_controller(&mut self, event_pump: &EventPump){
        //match event_pump.keyboard_state().keyboardstate{
        //      Scancode::W => println!("w pressed"),
        //      _ => ( )
        //}
        let binding = event_pump.keyboard_state();
        let iter = binding.scancodes();
        for scancode in iter{
            match scancode{
                //(Scancode::W, true) => self.pos.y -= 0.1,
                (Scancode::D, true) => self.alpha += 0.001,
                (Scancode::A, true) => self.alpha -= 0.001,
                //(Scancode::S, true) => self.pos.y += 0.1,

                _ => ( )
            }
        }
        return;
    }
    fn line_intersection(&self,line_seg_player:Vector2D<f64>,line_seg_start:&Vector2D<f64>, line_seg_end:&Vector2D<f64>) -> Option<Vector2D<f64>>{
        //Metodo consiste em calcular a intersecção entre as retas que compõe os segmentos e
        //utilizar propriedades dos valores t e u para descobrir se a intersecção está nos
        //segmentos de reta. Se não, devolvemos none.

        //PS: Eu preciso estudar mais algebra linear pra entender que merda tá acontecendo de
        //maneira menos superficial
        let x1 = self.pos.x; let y1 = self.pos.y;
        let x2 = line_seg_player.x; let y2 = line_seg_player.y;
        let x3 = line_seg_start.x; let y3 = line_seg_start.y;
        let x4 = line_seg_end.x; let y4 = line_seg_end.y;

        let t:f64 = ((x1-x3)*(y3-y4)-(y1-y3)*(x3-x4))/
                    ((x1-x2)*(y3-y4)-(y1-y2)*(x3-x4));


        let u:f64 = ((x1-x3)*(y1-y2)-(y1-y3)*(x1-x2))/
                    ((x1-x2)*(y3-y4)-(y1-y2)*(x3-x4));

        let t_vec:Vector2D<f64> = Vector2D::new(t,t);
        let u_vec:Vector2D<f64> = Vector2D::new(u,u); // transformamos em um vetor 2d para
        // conveniência da conta (a biblioteca não tanka multiplicar normal)

        //por alguma magia a linha criada será igual ao ponto de intersecção?
        let line_1: Vector2D<f64> = self.pos + Vector2D::mul_components(t_vec, line_seg_player - self.pos);
        //let line_2: Vector2D<f64> = line_seg_start + &Vector2D::mul_components(u_vec,line_seg_end - line_seg_start);

        if t >= 0. && t <= 1. && u >= 0. && u <= 1.{//significa que a interescção das retas se encontra dentro dos dois segmentos de reta
            return Some(line_1)
        }
        None
        //PSS: Obrigado Wikipedia (https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection)
    }
    pub fn calculate_distance(&self,line_seg_player:Vector2D<f64>,line_seg_start:&Vector2D<f64>, line_seg_end:&Vector2D<f64>) -> Option<f64>{
        let intersection_point:Option<Vector2D<f64>> = self.line_intersection(line_seg_player,line_seg_start, line_seg_end);
        match intersection_point{
            Some(intersection) => { 
                let distance:Vector2D<f64> = self.pos - intersection;
                return Some((distance.x.powf(2.) + distance.y.powf(2.)).sqrt().abs())
            },
            _ => { return None }
        }
    }
    //fn renderiza_setor(&self, setor:Setor)
    //fn renderiza_mapa(&self, mapa)

}
