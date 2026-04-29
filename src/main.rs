use macroquad::{color, prelude::*};
mod structs;
use structs::body;


#[macroquad::main("Grav Sim")]
async fn main() {
    let mut bodies: Vec<body> = Vec::new();
    let mut example_body = body{
mass: 100f32,
vel:  vec2(100f32, 100f32),
pos: vec2(10f32, 10f32),
color: RED
    };
    bodies.push(example_body);

    loop {
        clear_background(BLACK);
        for body in &bodies {
            let radius = body.mass.sqrt().clamp(1.0, 10.0);
            draw_circle(body.pos.x, body.pos.y, radius, body.color);
        }


        next_frame().await
    }
}
     





// Planning:

// I need a body object, with mass, and position

// Then every frame i need to calculate and move each object using gravity formula