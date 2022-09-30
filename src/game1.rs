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
    marker_speed: f32
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
            marker_speed: 5.0
        }
    }

    fn collides(&mut self) -> bool {
        if (self.y <= self.marker + 8.0 && self.y >= self.marker ) {
            return true ;
        } 
        else {
            return false;
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
        self.count += 1;

        if api.btn(Button::LEFT) {
            self.curr_speed += 0.1;
        }

        if api.btnp(Button::A) {
            if self.collides() {
                println!("Nuke Stopped")
            }
            else {
                println!("Test")
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
        //api.camera(self.x - 60.0, self.y - 60.0);
        api.cls(3);
        api.rect(5.0, 5.0, 15.0, 120.0, 10);

        api.rectfill(5.0,self.y, 15.0,5.0+self.y, 4);
        api.rectfill(15.0, self.marker, 20.0, 8.0+self.marker, 7);

       //api.spr(2, self.x,  20.0, 32.0, 32.0, true, true);
    }
}