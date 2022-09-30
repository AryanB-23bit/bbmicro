use crate::api::{BBMicroApi, BBMicroGame, Button};

use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Game1 {
    count: u32,
    x: f32,
    y: f32,
    vel: f32,
    curr_speed: f32,
    marker: f32, 
    marker_vel: f32,
    marker_speed: f32,
    // lives: u8,
    // message: String,
    joe_count: u8,
    vald_count: u8,
    game_lvl: u8,
    rng: ThreadRng

}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            count: 0,
            x: 100.0,
            y: 50.0,
            vel: 2.0,
            curr_speed: 2.0,
            marker: 50.0,
            marker_vel: 5.0,
            marker_speed: 5.0,
            // lives: 3,
            // message: String::from("You are tied"),
            joe_count: 0,
            vald_count: 0,
            game_lvl: 1,
            rng: rand::thread_rng()
        }
    }

    fn collides(&mut self) -> bool {
        if self.y <= self.marker + 8.0 && self.y >= self.marker{
            return true ;
        } 
        else {
            return false;
        }
    }

    fn draw_putin(&mut self, api: &mut BBMicroApi, x: f32, y:f32, w:f32, h:f32) {
        let rw = w  / 4.0;
        let rh = h / 5.0;

        for tx in 0..4 {
            for ty in 0..5 {   
                let n = tx + ty*16;     
                api.spr(n, x+(rw*(tx as f32)), y+(rh*ty as f32), rw, rh, false, false);
            }
        }
    }

    fn draw_joe(&mut self, api: &mut BBMicroApi, x: f32, y:f32, w:f32, h:f32) {
        let rw = w  / 4.0;
        let rh = h / 5.0;

        for tx in 0..4 {
            for ty in 0..5 {   
                let n = (4+tx) + ty*16;     
                api.spr(n, x+(rw*(tx as f32)), y+(rh*ty as f32), rw, rh, false, false);
            }
        }  
    }   

    fn draw_nuke(&mut self, api: &mut BBMicroApi, x: f32, y:f32, w:f32, h:f32) {
        let rw = w  / 4.0;
        let rh = h / 16.0;

        for tx in 0..4 {
            for ty in 0..16 {   
                let n = (8+tx) + ty*16;     
                api.spr(n, x+(rw*(tx as f32)), y+(rh*ty as f32), rw, rh, false, false);
            }
        }   
    }   
}

// enum Tiles {
//     Grass = 48,
//     WaterTL = 16,
//     WaterTR = 17,
//     WaterBL = 32,
//     WaterBR = 33,
//     WaterL = 34,
//     WaterR = 19,
//     Bird = 2
// }

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {
        // Draw a little island.
    
        //Play BGM
        //api.music("bgm", 0,0); //Uncomment after adding music.mp3
    }



    fn update(&mut self, api: &mut BBMicroApi) {
        if self.vald_count == 3 {
            return;
        }

        self.count += 1;

        if api.btn(Button::LEFT) {
            self.curr_speed += 0.1;
        }

        if api.btnp(Button::A) {
            if self.collides() || api.btn(Button::UP) {
                self.joe_count += 1;
                if self.joe_count == 3 {
                    // go to the next level
                    self.game_lvl += 1;
                    self.curr_speed = self.rng.gen_range(0.5..10.0); // make a random number;
                    self.marker_speed = self.rng.gen_range(0.5..10.0); //make a random number;
                    self.vel = self.curr_speed;
                    self.marker_vel = self.marker_speed;
                    if self.rng.gen::<f32>() > 0.5 {
                        self.vel *= -1.0;
                    }
                    if self.rng.gen::<f32>() > 0.5 {
                        self.marker_vel *= -1.0;
                    }
                    
                    self.joe_count = 0;
                    self.vald_count = 0;
                }
            }
            else {
                self.vald_count += 1;
            }
        }
        if self.x < 0.0 {
            self.x = 0.0;
        }

        if self.y >= 115.0 {
            self.vel = -self.curr_speed;
        } 
        if self.y < 5.0 {
            self.vel = self.curr_speed; 
        }

        self.y += self.vel;
        self.marker += self.marker_vel; 

        if self.marker >= 112.0 {
            self.marker_vel = -self.marker_speed;
        } 
        if self.marker < 5.0 {
            self.marker_vel = self.marker_speed;
        }
    }

    fn draw(&mut self, api: &mut BBMicroApi) {
        api.cls(4);     //api.camera(self.x - 60.0, self.y - 60.0);
        
        if self.vald_count == 3 {
            // TODO: draw the rocket
            api.print("YOU LOSE PUTIN SHOOTIN", 0.0, 10.0, false, 5.0, 5.0);
            self.draw_nuke(api, 50.0, 20.0, 1.0*32.0, 4.0*32.0);
            return;
        }
     
        api.rect(5.0, 5.0, 15.0, 120.0, 1);

        
        api.rectfill(5.0,self.y, 15.0,5.0+self.y, 8);
        //self.draw_putin(api, 5.0, self.y, 10.0, 5.0);
        api.rectfill(15.0, self.marker, 20.0, 8.0+self.marker, 7);
        

        for i in 0..self.joe_count {
            self.draw_joe(api,  60.0 + (i as f32)*20.0, 10.0, 16.0, 20.0);
        }

        for i in 0..self.vald_count {
            self.draw_putin(api,  60.0 + (i as f32)*20.0, 100.0, 16.0, 20.0);
        }

        // Use graphics to represent the level
        api.print("LEVEL ",  20.0, 60.0, false, 4.0, 4.0);

        //self.draw_putin(api, 50.0, 50.0, 16.0, 20.0)
       //api.spr(2, self.x,  20.0, 32.0, 32.0, true, true);
    }
}