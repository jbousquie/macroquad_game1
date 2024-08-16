use macroquad::prelude::load_texture;
use macroquad::prelude::Texture2D;
use macroquad::prelude::GREEN;
use rand::Rng;

use crate::game::Sprite;

const BLOCK_NB: i32 = 50;
const BLOCK_SIZE: i32 = 30;

pub async fn create_world() -> Vec<Sprite> {
    let mut blocks: Vec<Sprite> = Vec::new();
    let max_nb_x = 30;
    let max_nb_y = 20;
    let mut rng = rand::thread_rng();
    for _i in 0..BLOCK_NB {
        let x = (rng.gen_range(0..max_nb_x) * BLOCK_SIZE) as f32;
        let y = (rng.gen_range(0..max_nb_y) * BLOCK_SIZE) as f32;
        let block = Sprite {
            x,
            y,
            width: BLOCK_SIZE as f32,
            height: BLOCK_SIZE as f32,
            texture_id: 0,
            color: GREEN,
        };
        blocks.push(block);
    }
    blocks
}
