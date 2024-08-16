use game::{draw_sprites, ysort_sprites};
use macroquad::prelude::*;
use std::rc::Rc;
mod game;
mod player;
mod world;
use player::player::move_player;
use world::world::create_world;

#[macroquad::main("Game1")]
async fn main() {
    let mut sprites: Vec<game::Sprite> = Vec::new();
    let texture_happy = Rc::new(load_texture("images/rustacean_happy.png").await.unwrap());
    sprites = create_world(sprites, &texture_happy);

    let player_sprite = game::Sprite {
        id: 1000,
        x: 150.0,
        y: 150.0,
        width: 50.0,
        height: 50.0,
        texture: texture_happy.clone(),
        color: WHITE,
    };
    sprites.push(player_sprite);
    let mut x = 150.0;
    let mut y = 150.0;

    loop {
        clear_background(GRAY);
        (x, y) = move_player(x, y);
        for sprite in &mut sprites {
            if sprite.id == 1000 {
                sprite.move_to(x, y);
            }
        }
        sprites = ysort_sprites(sprites);
        draw_sprites(&sprites);

        next_frame().await
    }
}
