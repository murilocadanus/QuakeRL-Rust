extern crate graphics;

use render::{Render, Draw};
use sprite::Sprite;
use std::vec::Vec;
use piston::graphics::*;
use current::Set;

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
    pub tileset: Sprite,
    pub map: Vec<uint>,
    pub tiles: Vec<Image>,

    x_size: uint,
    y_size: uint,
}

impl TileMap {
    pub fn from_tileset_path(path: &Path) -> TileMap {
        TileMap {
            // Initialize the tileset
            tileset: Sprite::from_path(path),

            // Initialize the tiles
            tiles: Vec::new(),

            // Initialize common values
            x_size: 0u,
            y_size: 0u,

            // Initialize the vector of map
            map: Vec::new(),
        }
    }

    pub fn build_procedural_map(&mut self, width: uint, height: uint) -> Vec<uint>
    {
        // Create the map
        self.x_size = width;
        self.y_size = height;

        // Ugly, try to use an iterator rather then line/column loop algorithm
        // Create the borders and fill the midle with dirt
        for y in range(0u, self.y_size) {
            for x in range(0u, self.x_size) {
                //Making the borders of unwalkable walls
                if y == 0u {
                    self.map.push(Tiles::TileStoneWall as uint);
                }
                else if y == self.y_size - 1 {
                    self.map.push(Tiles::TileStoneWall as uint);
                }
                else if x == 0u {
                    self.map.push(Tiles::TileStoneWall as uint);
                }
                else if x == self.x_size - 1u {
                    self.map.push(Tiles::TileStoneWall as uint);
                }
                //Fill the rest with floor or dirt
                else {
                    self.map.push(Tiles::TileBrickFloor as uint);
                }
            }
        }

        // Ugly, try to return a reference instead
        self.map.clone()
    }

}

impl Draw for TileMap {
    fn draw(&self, render: &mut Render) {

        // Ugly, try to use an iterator rather then line/column loop algorithm
        for y in range(0u, self.y_size) {
            for x in range(0u, self.x_size) {
                let sprite_context = &render.ctx.trans((x * 40) as f64, (y * 40) as f64);
                Image::new().set(SrcRect([0, (self.map[x + self.x_size * y] as i32) * 40, 40, 40])).draw(&self.tileset.image, sprite_context, &mut render.gl);
            }
        }

    }
}

