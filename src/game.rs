use macroquad::prelude::*;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub texture_id: usize,
    pub color: Color,
}

pub fn ysort_sprites(mut sprites: Vec<Sprite>) {
    sprites.sort_by(|a, b| (cmp_sprites(&a, &b)));
}

fn cmp_sprites(a: &Sprite, b: &Sprite) -> std::cmp::Ordering {
    (a.y as i32).cmp(&(b.y as i32))
}
pub fn cull_sprites() {}

pub fn draw_sprites(sprites: Vec<Sprite>, textures: Vec<Texture2D>) {
    for sprite in sprites {
        draw_texture(
            &textures[sprite.texture_id],
            sprite.x,
            sprite.y,
            sprite.color,
        );
    }
}
