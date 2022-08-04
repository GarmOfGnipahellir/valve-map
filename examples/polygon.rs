use macroquad::prelude::*;

const POINT_SIZE: f32 = 8.0;
const POINT_SIZE_SQ: f32 = POINT_SIZE * POINT_SIZE;

#[macroquad::main("Valve Map")]
async fn main() {
    let mut points = Vec::new();
    let mut selected = None;

    loop {
        let mouse_pos = {
            let mouse_pos = mouse_position();
            vec2(mouse_pos.0, mouse_pos.1).extend(0.0)
        };
        let over_point = is_over_point(&points, mouse_pos);

        if is_mouse_button_pressed(MouseButton::Left) {
            selected = over_point;
            if selected.is_none() {
                let i = points.len();
                points.push(mouse_pos);
                selected = Some(i);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            selected = None;
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(i) = selected {
                points[i] = mouse_pos;
            }
        }

        if is_mouse_button_released(MouseButton::Right) {
            if let Some(i) = over_point {
                points.remove(i);
            }
        }

        clear_background(BLACK);

        if let Some(ordered_points) = order_points(&points) {
            for i in 0..ordered_points.len() {
                let cp = ordered_points[i];
                let np = ordered_points[(i + 1) % ordered_points.len()];

                draw_line(cp.x, cp.y, np.x, np.y, 2.0, WHITE);
            }
        }

        for (i, p) in points.iter().enumerate() {
            let color = over_point
                .or(selected)
                .and_then(|x| if i == x { Some(YELLOW) } else { None })
                .unwrap_or(RED);
            draw_circle(p.x, p.y, POINT_SIZE, color);
        }

        next_frame().await
    }
}

fn is_over_point(points: &Vec<Vec3>, pos: Vec3) -> Option<usize> {
    for (i, p) in points.iter().enumerate() {
        if p.distance_squared(pos) <= POINT_SIZE_SQ {
            return Some(i);
        }
    }
    None
}

fn order_points(points: &Vec<Vec3>) -> Option<Vec<Vec3>> {
    if points.len() <= 3 {
        return None;
    }

    let center = points.iter().fold(Vec3::ZERO, |acc, p| acc + p.clone()) / points.len() as f32;

    let mut ordered = points.clone();
    for n in 0..ordered.len() - 2 {
        let a = (ordered[n] - center).normalize();
        let p = Vec3::Z.cross(a);

        let mut smallest_angle = -1.0;
        let mut smallest = usize::MAX;

        for m in n + 1..ordered.len() {
            let b = (ordered[m] - center).normalize();
            if p.dot(b) > 0.0 {
                let angle = a.dot(b);
                if angle > smallest_angle {
                    smallest_angle = angle;
                    smallest = m;
                }
            }
        }

        ordered.swap(n + 1, smallest);
    }

    Some(ordered)
}
