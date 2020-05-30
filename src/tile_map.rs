/*
   _____ _                _____ _                                                      
  / ____| |              / ____| |                                        _            
 | (___ | |_ __ _ _ __  | (___ | | __ _ _ __ ___  _ __ ___   ___ _ __ ___(_)           
  \___ \| __/ _` | '__|  \___ \| |/ _` | '_ ` _ \| '_ ` _ \ / _ \ '__/ __|             
  ____) | || (_| | |     ____) | | (_| | | | | | | | | | | |  __/ |  \__ \_            
 |_____/ \__\__,_|_|    |_____/|_|\__,_|_| |_| |_|_| |_| |_|\___|_|  |___(_)           
  _____        __ _       _ _                     _                 _                  
 |_   _|      / _(_)     (_) |           /\      | |               | |                 
   | |  _ __ | |_ _ _ __  _| |_ ___     /  \   __| |_   _____ _ __ | |_ _   _ _ __ ___ 
   | | | '_ \|  _| | '_ \| | __/ _ \   / /\ \ / _` \ \ / / _ \ '_ \| __| | | | '__/ _ \
  _| |_| | | | | | | | | | | ||  __/  / ____ \ (_| |\ V /  __/ | | | |_| |_| | | |  __/
 |_____|_| |_|_| |_|_| |_|_|\__\___| /_/    \_\__,_| \_/ \___|_| |_|\__|\__,_|_|  \___|
*/


#![allow(non_snake_case)]

use crate::tile;

use std::io::prelude::*;
use std::fs::File;

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
