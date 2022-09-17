use std::f32::consts::PI;

use macroquad::{prelude::*};

fn dist (x1:f32, y1:f32, x2:f32, y2:f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    return (dx * dx + dy * dy).sqrt();
}

fn intersects(a:f32,b:f32,c:f32,d:f32,p:f32,q:f32,r:f32,s:f32) -> bool {
    let (det, gamma, lambda);
    det = (c - a) * (s - q) - (r - p) * (d - b);
    if det == 0.0 {
        return false;
    }
    lambda = ((s - q) * (r - a) + (p - r) * (s - b)) / det;
    gamma = ((b - d) * (r - a) + (c - a) * (s - b)) / det;
    (0.0 < lambda && lambda < 1.0) && (0.0 < gamma && gamma < 1.0)
}

fn ray (x:f32, y:f32, dir:f32, clip:f32, walls:[[f32; 4]; 5]) -> (f32, i32) {
    let mut closest: f32 = clip;
    let mut wall_used: i32 = 0;
    for i in 0..4 {
		
		// This section is Iron Programming's code

		// shortening variables
		let wall: [f32; 4] = walls[i];
        let (x1, y1, x2, y2) = (wall[0], wall[1], wall[2], wall[3]);
        let (x3, y3, x4, y4) = (x, y, x+clip*cos_deg(dir), y+clip*-sin_deg(dir));
            
        // returns true if the line from (a,b)->(c,d) intersects with (p,q)->(r,s)
        if intersects(x1, y1, x2, y2, x3, y3, x4, y4) {
        
	        // denominator
	        let den: f32 = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
	        if den == 0.0 {
	            return (0.0, 0);
	        }
	        
	        let t: f32 = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
	        let u: f32 = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;
	        
	        // Does the ray collide with a wall?
	        if t > 0.0 && t < 1.0 && u > 0.0 {
	            // If true, then where does it intersect?
	            let ptx: f32 = x1 + t * (x2 - x1);
	            let pty: f32 = y1 + t * (y2 - y1);
	            let d = dist(x3, y3, ptx, pty); // distance betwen 2 points
	            if d < closest {
					closest = d;
                    wall_used = i as i32;
				}
	        }
        }
        // This is the end of Iron Programming's code
	}
	if closest == clip {
	    return (0.0, 0);
	}
	return (closest, wall_used);
}

fn cos_deg (deg:f32) -> f32 {
    return (deg * ( PI / 180.0 )).cos();
}

fn sin_deg (deg:f32) -> f32 {
    return (deg * ( PI / 180.0 )).sin();
}

fn ray_walk (x:f32, y:f32, dir:f32, walls: [[f32; 4]; 5]) -> bool {
    let check = ray(x, y, dir, 10.0, walls);
    //return check == (0.0, 0);
    return true;
}

#[macroquad::main("Raycaster")]
async fn main() {

    let walls: [[f32; 4]; 5] = [[10.0, 10.0, 390.0, 10.0], [390.0, 10.0, 390.0, 390.0], [390.0, 390.0, 10.0, 390.0], [10.0, 390.0, 10.0, 10.0], [200.0, 40.0, 300.0, 300.0]];
    let mut wall_lighting: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let width = screen_width();
    let height = screen_height();
    let fov = 66.0;
    let angle_increment = fov/width;
    let mut angle: f32 = 0.0;
    let mut x = 20.0;
    let mut y = 20.0;

    for i in 0..4 {
        let ex = walls[i][0];
        let ey = walls[i][1];
        let cx = walls[i][2];
        let cy = walls[i][3];
        let dy = ey - cy;
        let dx = ex - cx;
        let mut theta = dy.atan2(dx); // range (-PI, PI]
        theta *= 180.0 / PI; // rads to degs, range (-180, 180]
        if theta < 0.0 {
            //theta = 360.0 + theta; // range [0, 360)
            theta = theta.abs();
        }
        wall_lighting[i] = theta / 360.0;
    }

    loop {
        if is_key_down(KeyCode::W) {
            if ray_walk(x, y, angle - fov*0.5, walls) {
                x += 1.0*cos_deg(angle);
                y -= 1.0*sin_deg(angle);
            }
        }
        if is_key_down(KeyCode::D) {
            if ray_walk(x, y, angle - fov*0.5 - 90.0, walls) {
                x = x+1.0*cos_deg(angle-90.0);
                y = y-1.0*sin_deg(angle-90.0);
            }
        }
        if is_key_down(KeyCode::S) {
            if ray_walk(x, y, angle - fov*0.5 + 180.0, walls) {
                x = x+1.0*cos_deg(angle+180.0);
                y = y-1.0*sin_deg(angle+180.0);
            }
        }
        if is_key_down(KeyCode::A) {
            if ray_walk(x, y, angle - fov*0.5 + 90.0, walls) {
                x = x+1.0*cos_deg(angle+90.0);
                y = y-1.0*sin_deg(angle+90.0);
            }
        }
        if is_key_down(KeyCode::Left) {
            angle += 1.0;
        }
        if is_key_down(KeyCode::Right) {
            angle -= 1.0;
        }
        clear_background(Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 });
        draw_rectangle(0.0, screen_height()*0.5, screen_width(), screen_height()*0.5, GREEN);
        for i in 0..width as i32{
            let ray = ray(x, y, (angle - fov*0.5) + i as f32 * angle_increment, 600.0, walls);
            let dist = ray.0;
            let line_height = 10.0*width/dist/cos_deg((i as f32-width/2.0)*angle_increment);
            let col = wall_lighting[ray.1 as usize];

            draw_line(
                width - i as f32, ((height/2.0)+line_height) as f32,
                width - i as f32, ((height/2.0)-line_height) as f32,
                1.0, Color { r: col, g: col, b: col, a: 1.0 }
            );

            for i in 0..4 {
                let wall = walls[i];
                draw_line(wall[0], wall[1], wall[2], wall[3], 4.0, BLACK);
                draw_circle(x, y, 5.0, RED);
                draw_triangle(vec2(x, y), vec2(x+10.0*cos_deg(-33.0), y+10.0*cos_deg(-33.0)), vec2(x-10.0*cos_deg(-33.0), y-10.0*cos_deg(-33.0)), YELLOW);
            }
        }
        next_frame().await;
    }
}