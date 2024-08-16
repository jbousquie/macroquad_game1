use macroquad::prelude::*;
mod game;
mod player;
mod world;
use player::player::move_player;
use world::world::create_world;

const BLOCK_SIZE: f32 = 30.0;

#[macroquad::main("Game1")]
async fn main() {
    let blocks = create_world();
    let textures = vec![load_texture("images/rustacean_happy.png").await.unwrap()];

    let player_sprite = game::Sprite {
        x: 150.0,
        y: 150.0,
        width: 30.0,
        height: 30.0,
        texture_id: 0,
        color: WHITE,
    };

    let mut x = 150.0;
    let mut y = 150.0;
    loop {
        clear_background(GRAY);
        (x, y) = move_player(x, y);
        draw_texture(&texture, x, y, WHITE);
        for block in &blocks {
            draw_rectangle(
                block.x as f32,
                block.y as f32,
                BLOCK_SIZE,
                BLOCK_SIZE,
                GREEN,
            );
        }
        next_frame().await
    }
}
