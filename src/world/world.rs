use macroquad::prelude::Texture2D;
use macroquad::prelude::GREEN;
use rand::Rng;
use std::rc::Rc;

use crate::game::Sprite;

const BLOCK_NB: i32 = 50;
const BLOCK_SIZE: i32 = 40;

pub fn create_world(mut sprites: Vec<Sprite>, texture: &Rc<Texture2D>) -> Vec<Sprite> {
    let max_nb_x = 30;
    let max_nb_y = 20;
    let mut rng = rand::thread_rng();
    for i in 0..BLOCK_NB {
        let x = (rng.gen_range(0..max_nb_x) * BLOCK_SIZE) as f32;
        let y = (rng.gen_range(0..max_nb_y) * BLOCK_SIZE) as f32;
        let block = Sprite {
            id: i as u32,
            x,
            y,
            width: BLOCK_SIZE as f32,
            height: BLOCK_SIZE as f32,
            texture: texture.clone(),
            color: GREEN,
        };
        sprites.push(block);
    }
    sprites
}
