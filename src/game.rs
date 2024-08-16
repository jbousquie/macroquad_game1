use macroquad::prelude::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct Sprite {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub texture: Rc<Texture2D>,
    pub color: Color,
}

impl Sprite {
    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Sprite {
        self.x = x;
        self.y = y;
        self
    }
}

pub fn ysort_sprites(mut sprites: Vec<Sprite>) -> Vec<Sprite> {
    sprites.sort_by(|a, b| (cmp_sprites(&a, &b)));
    sprites
}

fn cmp_sprites(a: &Sprite, b: &Sprite) -> std::cmp::Ordering {
    ((a.y + a.height) as i32).cmp(&((b.y + b.height) as i32))
}
//pub fn cull_sprites() {}

pub fn draw_sprites(sprites: &Vec<Sprite>) {
    for sprite in sprites {
        draw_texture_ex(
            &sprite.texture,
            sprite.x,
            sprite.y,
            sprite.color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(sprite.width, sprite.height)),
                ..Default::default()
            },
        );
    }
}
