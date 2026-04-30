use macroquad::{color, prelude::*};
mod structs;
use structs::body;


#[macroquad::main("Grav Sim")]
async fn main() {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    let center = vec2(center_x, center_y);
    let mut bodies: Vec<body> = Vec::new();
    let  example_body = body{
mass: 10000f32,
vel:  vec2(0f32, 0f32),
pos: center,
color: YELLOW,
    };
    let lil_object_pos = vec2(250f32, 250f32);
    let  example_body2 = body{
        mass: 10f32,
        vel:  calculate_orbit(lil_object_pos, center, 10000f32),
        pos: lil_object_pos,
        color: BROWN,
            };
    bodies.push(example_body);
    bodies.push(example_body2);



    loop {
        clear_background(BLACK);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            bodies.push(body { mass: (100f32), vel: (vec2(0f32, 0f32)), pos: (vec2(x, y)), color: (RED) })
        }

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
     


fn calculate_orbit(body_pos: Vec2, orbit_pos: Vec2, orbit_mass: f32) -> Vec2{
    let offset = body_pos - orbit_pos; 
    let dist_sq = offset.length_squared();
    let dist = dist_sq.sqrt();

    let accel = (50f32 * orbit_mass) / (dist_sq + 100f32);

    let speed = (accel * dist).sqrt();

    let tangent = vec2(-offset.y, offset.x).normalize();

    let orbit_vel = tangent * speed;
    return orbit_vel;

}


