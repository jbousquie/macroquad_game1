use macroquad::prelude::*;
const PLAYER_SPEED: f32 = 6.0;
fn move_right(x: f32) -> f32 {
    x + PLAYER_SPEED
}

fn move_left(x: f32) -> f32 {
    x - PLAYER_SPEED
}

fn move_up(y: f32) -> f32 {
    y - PLAYER_SPEED
}

fn move_down(y: f32) -> f32 {
    y + PLAYER_SPEED
}

pub fn move_player(x: f32, y: f32) -> (f32, f32) {
    let mut x = x;
    let mut y = y;
    if is_key_down(KeyCode::Right) {
        x = move_right(x);
    }
    if is_key_down(KeyCode::Left) {
        x = move_left(x);
    }
    if is_key_down(KeyCode::Up) {
        y = move_up(y);
    }
    if is_key_down(KeyCode::Down) {
        y = move_down(y);
    }
    (x, y)
}
