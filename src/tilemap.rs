extern crate graphics;
use render::{Render, Draw};
use std::vec::Vec;
use opengl_graphics::Texture;
use quack::Set;
use graphics::{Image, SrcRect, RelativeTransform};

#[allow(dead_code)]
enum Tiles {
    TileBrickFloor,
    TileBrickRoomFloor,
    TileStoneWall,
    TileMetalFloor,
    TileCorridor,
    TileDoor,
    TileUpStairs,
    TileDownStairs,
    TileChest
}

pub struct TileMap {
    pub tileset: Texture,
    pub map: Vec<u32>,
    pub tiles: Vec<Image>,

    x_size: u32,
    y_size: u32,
}

impl TileMap {
    pub fn from_tileset_path(path: &Path) -> TileMap {
        TileMap {
            // Initialize the tileset
            tileset: Texture::from_path(path).unwrap(),

            // Initialize the tiles
            tiles: Vec::new(),

            // Initialize common values
            x_size: 0,
            y_size: 0,

            // Initialize the vector of map
            map: Vec::new(),
        }
    }

    pub fn build_procedural_map(&mut self, width: u32, height: u32) -> Vec<u32>
    {
        // Create the map
        self.x_size = width;
        self.y_size = height;

        // Ugly, try to use an iterator rather then line/column loop algorithm
        // Create the borders and fill the midle with dirt
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                //Making the borders of unwalkable walls
                if y == 0 {
                    self.map.push(Tiles::TileStoneWall as u32);
                }
                else if y == self.y_size - 1 {
                    self.map.push(Tiles::TileStoneWall as u32);
                }
                else if x == 0 {
                    self.map.push(Tiles::TileStoneWall as u32);
                }
                else if x == self.x_size - 1 {
                    self.map.push(Tiles::TileStoneWall as u32);
                }
                //Fill the rest with floor or dirt
                else {
                    self.map.push(Tiles::TileBrickFloor as u32);
                }
            }
        }

        // Ugly, try to return a reference instead
        self.map.clone()
    }

}

impl Draw for TileMap {
    fn draw(&self, at: &[f64; 2], render: &mut Render) {

        // Ugly, try to use an iterator rather then line/column loop algorithm
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let sprite_context = &render.ctx.trans(at[0] + (x * 40) as f64, at[1] + (y * 40) as f64);
                Image::new().set(SrcRect([0, (self.map[(x + self.x_size * y) as usize] as i32) * 40, 40, 40])).draw(&self.tileset, sprite_context, &mut render.gl);
            }
        }
    }
}

