use macroquad::{color, prelude::*};
mod structs;
use structs::body;


#[macroquad::main("Grav Sim")]
async fn main() {
    let mut bodies: Vec<body> = Vec::new();
    let  example_body = body{
mass: 1000f32,
vel:  vec2(0f32, 0f32),
pos: vec2(100f32, 100f32),
color: YELLOW,
    };
    let  example_body2 = body{
        mass: 100f32,
        vel:  vec2(0f32, 0f32),
        pos: vec2(250f32, 250f32),
        color: BROWN,
            };
    bodies.push(example_body);
    bodies.push(example_body2);



    loop {
        clear_background(BLACK);
        let dt = get_frame_time();
        update_physics(&mut bodies, dt);
        for body in &bodies {
            let radius = body.mass.sqrt().clamp(1.0, 100.0);
            draw_circle(body.pos.x, body.pos.y, radius, body.color);
        }


        next_frame().await
    }
}

fn update_physics(bodies: &mut Vec<body>, dt: f32) {

    let old_state = bodies.clone();
    

    for i in bodies.iter_mut() {
        let mut total_acceleration = vec2(0.0, 0.0);
        for x in &old_state {
            let dir = x.pos - i.pos;
            let dist_sq = dir.length_squared();

            if dist_sq > 0.001 {
                let dist = dist_sq.sqrt();
                let force_mag = (50f32 * x.mass) / (dist_sq + 100f32);
                total_acceleration += (dir / dist) * force_mag;
            }
        }

        i.vel += total_acceleration * dt;
        i.pos += i.vel * dt;
    }
}
     






// Planning:

// I need a body object, with mass, and position

// Then every frame i need to calculate and move each object using gravity formula