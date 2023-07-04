
pub struct Particle {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
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
            particle.x += particle.vx;
            particle.y += particle.vy;
            particle.vy += 1;
        }
        //update the clicks
        for click in self.clicks.iter_mut() {
            match click.clic_type {
                ClicType::Sand => {
                    self.particles.push(Particle {
                        x: click.x,
                        y: click.y,
                        vx: 0,
                        vy: 0,
                        color: String::from("yellow"),
                    });
                }
                ClicType::Water => {
                    self.particles.push(Particle {
                        x: click.x,
                        y: click.y,
                        vx: 0,
                        vy: 0,
                        color: String::from("blue"),
                    });
                }
                ClicType::Wall => {
                    self.particles.push(Particle {
                        x: click.x,
                        y: click.y,
                        vx: 0,
                        vy: 0,
                        color: String::from("black"),
                    });
                }
            }
        }
        //clear the clicks
        self.clicks.clear();
    }
}
