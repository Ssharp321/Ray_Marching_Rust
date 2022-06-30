use nannou::prelude::*;
//use std::thread;
static CIRCLES: [[f64;3]; 3] = [[0.0,0.0,150.0],[-300.0,0.0,100.0],[300.0,300.0,40.0]];
static PI: f64 = 3.14159265359;
fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    let mut ray_cout: i64 = 10_000;
    let inital_dist: f64 = true_dist([app.mouse.x as f64,app.mouse.y as f64]);

    // Clear the background to purple.
    draw.background().color(PLUM);

    //shape in middle
    for circle in CIRCLES {
        draw.ellipse()
            .color(DARKVIOLET)
            .w(circle[2] as f32 * 2.0)
            .h(circle[2] as f32 * 2.0)
            .x_y(circle[0] as f32, circle[1] as f32);
    }

    let mut ray: [f64;2];

    for i in 0..ray_cout{
        ray = shoot_ray([app.mouse.x as f64,app.mouse.y as f64],inital_dist,(i as f64 /ray_cout as f64)*360.0);
        draw.line()
            .start(pt2(app.mouse.x, app.mouse.y))
            .end(pt2(ray[0] as f32, ray[1] as f32))
            .weight(1.0)
            .color(STEELBLUE);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn true_dist(positon: [f64;2]) -> f64{
    let mut out: f64 = 10000000000.0;
    let mut dist: f64 = 0.0;
    for i in CIRCLES{
        dist = get_dist([i[0],i[1]],positon) - i[2];
        if dist < out{
            out = dist;
        }
    }
    return out
}

fn get_dist(object: [f64; 2], target: [f64; 2]) -> f64 {
    f64::sqrt((object[0] - target[0]).powf(2.0) + (object[1] - target[1]).powf(2.0))
}

fn angle_to_rad(angle: f64) -> f64 {
    return angle / 180.0 * PI;
}

fn shoot_ray(ray_pos: [f64; 2], inital_dist: f64, ray_angle: f64) -> [f64; 2] {
    let mut dist = inital_dist;
    let mut ray_x = ray_pos[0];
    let mut ray_y = ray_pos[1];
    let cos_angle = angle_to_rad(ray_angle).cos();
    let sin_angle = angle_to_rad(ray_angle).sin();
    while dist > 0.01 && dist < 5000.0 {
        ray_x = ray_x + (dist * sin_angle);
        ray_y = ray_y + (dist * cos_angle);
        dist = true_dist([ray_x,ray_y]);
    }
    return [ray_x, ray_y];
}
