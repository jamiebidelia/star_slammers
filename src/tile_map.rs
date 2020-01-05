use crate::tile;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;

pub const MAX_LEN_X : usize = 256;
pub const MAX_LEN_Y : usize = 256;

pub struct TileMap
{
    x_size : u32,
    y_size : u32,

    map : Vec<Vec<tile::Tile>>,      // The first dimension will be x, the second will be y.
}

impl TileMap
{
    pub fn get_x_size(&self) -> &u32
    {
        &self.x_size
    }

    pub fn set_x_size(&mut self, new_x_size : u32)
    {
        self.x_size = new_x_size;
    }

    pub fn get_y_size(&self) -> &u32
    {
        &self.y_size
    }

    pub fn set_y_size(&mut self, new_y_size : u32)
    {
        self.y_size = new_y_size;
    }

    pub fn get_map(&self) -> &Vec<Vec<tile::Tile>>
    {
        &self.map
    }
    
    pub fn get_map_handle(&mut self) -> &mut Vec<Vec<tile::Tile>>
    {
        &mut self.map
    }
}

pub fn load_map(file_name : String, tile_database: &[tile::Tile; tile::MAX_TILES]) -> TileMap
{
    let path = std::path::Path::new(&file_name);
    let file = File::open(&path).expect("Cannot Open File!");
    let buffer = std::io::BufReader::new(file);
    
    
    let mut y_vector = Vec::new();
    for line in buffer.lines()
    {
        let mut x_vector = Vec::new();
        let line = line.expect("Could Not Read Line!");
        for entry in line.chars()
        {
            
            let tile = tile::get_tile_by_image(&entry, &tile_database).expect("Could not find tile image");
            x_vector.push(tile);
        }
        y_vector.push(x_vector);
    }
    
    TileMap{x_size : y_vector[0].len() as u32,
            y_size : y_vector.len() as u32,
            map    : y_vector}
}
