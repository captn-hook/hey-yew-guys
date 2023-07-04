#[allow(unused)]
use log::info;

pub struct Particle {
    pub x: i32,
    pub y: i32,
    pub vx: f64,
    pub vy: f64,
    pub color: String,
}

#[derive(Debug)]
pub struct Pixel {
    pub x: i32,
    pub y: i32,
    pub color: String,
}

pub enum ClicType {
    Sand,
    Water,
    Wall,
}

pub struct Click {
    pub x: i32,
    pub y: i32,
    pub clic_type: ClicType,
}

pub enum Msg {
    //this is the message that is sent when the user clicks on the canvas
    Click(Click),
}

pub struct SandGame {
    pub width: i32,
    pub height: i32,
    pub particles: Vec<Particle>,
    pub clicks: Vec<Click>,
}

impl SandGame {
    pub fn update(&mut self) {
        //update the particles
        for particle in self.particles.iter_mut() {
            if particle.x < 0 || particle.x > self.width {
                particle.vx = -particle.vx;
            } else {
                particle.x += particle.vx as i32;
            }

            if particle.y < 0 || particle.y > self.height {
                particle.vy = -particle.vy;
            } else {
                particle.y += particle.vy as i32;
            }
            
            particle.vy += 1.0;
            particle.vx *= 0.99;
            particle.vy *= 0.99;
        }
        //update the clicks
        for click in self.clicks.iter_mut() {
            match click.clic_type {
                ClicType::Sand => {
                    self.particles.push(Particle {
                        x: click.x,
                        y: click.y,
                        vx: 0.0,
                        vy: 0.0,
                        color: String::from("yellow"),
                    });
                }
                ClicType::Water => {
                    self.particles.push(Particle {
                        x: click.x,
                        y: click.y,
                        vx: 0.0,
                        vy: 0.0,
                        color: String::from("blue"),
                    });
                }
                ClicType::Wall => {
                    self.particles.push(Particle {
                        x: click.x,
                        y: click.y,
                        vx: 0.0,
                        vy: 0.0,
                        color: String::from("black"),
                    });
                }
            }
        }
        //clear the clicks
        self.clicks.clear();
    }

    //returns a list of pixels coords and their color
    pub fn get_pixels(&self) -> Vec<Pixel> {
        let mut pixels: Vec<Pixel> = Vec::new();
        for particle in self.particles.iter() {
            pixels.push(Pixel {
                x: particle.x,
                y: particle.y,
                color: particle.color.clone(),
            });
        }
        pixels
    }
}
        