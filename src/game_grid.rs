use std::{collections::HashMap, ops::{Add, Sub}};
use bevy::prelude::Vec3;

pub struct GameGrid {
    pub grid: HashMap<GridCell, Option<GridCellType>>,
}

impl GameGrid{
    pub fn new(widht: i32, length: i32, height: i32) -> Self {
        let mut grid:HashMap<GridCell, Option<GridCellType>> = HashMap::new();
        for x in 0..widht{
            for y in 0..height{
                for z in 0..length{
                    grid.insert(GridCell{x,y,z}, None);
                }
            }
        }
        GameGrid { grid }
    }

    pub fn reset(&mut self, widht: i32, length: i32, height: i32){
        self.grid.clear();
        for x in 0..widht{
            for y in 0..height{
                for z in 0..length{
                    self.grid.insert(GridCell{x,y,z}, None);
                }
            }
        }
    }

    pub fn tiles_from_csv(mut self, tiles: Vec<String>) -> Self {
        for (y, csv) in tiles.iter().enumerate(){
            for(z, line) in csv.lines().enumerate(){
                let indexes = line.split(",").collect::<Vec<&str>>();
                for(x, str_index) in indexes.into_iter().enumerate(){
                    let index = str_index.parse::<i32>().unwrap();
                    if index < 0 { continue; }
                    let cell = GridCell{x: x as i32, y: y as i32, z: z as i32};
                    let value = self.grid.get_mut(&cell).unwrap();
                    *value = Some(GridCellType::Tile(index));
                }
            }
        }
        self
    } 
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash,)]
pub struct GridCell {
    pub x: i32, 
    pub y: i32, 
    pub z: i32,
}

impl GridCell {
    pub fn new(x: i32, y: i32, z: i32) -> Self{
        Self { x, y, z }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
    
    pub fn out_of_bounds(&self, min: GridCell, max: GridCell) -> bool{
        self.x < min.x || self.y < min.y || self.z < min.z || self.x > max.x || self.y > max.y || self.z > max.z
    }
}

impl Add for GridCell {
    type Output = GridCell;
    
    fn add(self, other: GridCell) -> GridCell{
        GridCell { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }

}

impl Sub for GridCell {
    type Output = GridCell;
    
    fn sub(self, other: GridCell) -> GridCell{
        GridCell { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }

}


#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash,)]
pub enum GridCellType {
    Tile(i32),
    Player,
    MovingTile(i32),
}