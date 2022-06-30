use nannou::prelude::*;
static CIRCLE: [f32;3] = [0.0, 0.0, 150.0];
static PI: f64 = 3.14159265359;
static RAYCOUNT: i64 = 10_000;
fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    let dist = get_dist([CIRCLE[0] as f64,CIRCLE[1] as f64],[app.mouse.x as f64, app.mouse.y as f64])-CIRCLE[2] as f64;

    //println!("{}",dist);

    //shape in middle
    draw.ellipse()
    .color(DARKVIOLET)
    .w(CIRCLE[2]*2.0)
    .h(CIRCLE[2]*2.0)
    .x_y(CIRCLE[0], CIRCLE[1]);

    
    let mut ray_pos: [f64;2];

    for i in 0..RAYCOUNT{
        ray_pos = shoot_ray([app.mouse.x as f64, app.mouse.y as f64],dist,(i as f64/RAYCOUNT as f64)*360.0);
        draw.line()
        .start(pt2(app.mouse.x, app.mouse.y))
        .end(pt2(ray_pos[0] as f32,ray_pos[1] as f32))
        .weight(1.0)
        .color(STEELBLUE);
    }
    

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn get_dist(object: [f64; 2],target: [f64; 2]) -> f64{
    f64::sqrt((object[0]-target[0]).powf(2.0) +(object[1]-target[1]).powf(2.0))
}

fn angle_to_rad(angle: f64) -> f64{
    return angle/180.0*PI
}

fn shoot_ray(ray_pos: [f64;2],inital_dist: f64,ray_angle: f64) -> [f64; 2]{
    let mut dist = inital_dist;
    let mut ray_x = ray_pos[0];
    let mut ray_y = ray_pos[1];
    let cos_angle = angle_to_rad(ray_angle).cos();
    let sin_angle = angle_to_rad(ray_angle).sin(); 
    while dist > 0.01 && dist < 5000.0{
        ray_x = ray_x+(dist*sin_angle);
        ray_y = ray_y+(dist*cos_angle);
        dist = get_dist([CIRCLE[0] as f64,CIRCLE[1] as f64],[ray_x,ray_y])-CIRCLE[2] as f64;
    }
    return [ray_x,ray_y]
}
    