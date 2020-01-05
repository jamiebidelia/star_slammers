

pub const MAX_TILES : usize = 256; // Keep this as a usize so we can use it to index the tile_database.


#[derive(Copy, Clone)]         // We are not going to fill up all of the tiles with data until game needs them.
pub struct Tile
{
    passable : bool,
    damage   : i32,
    image    : char
}

impl Tile
{
    pub fn new() -> Tile
    {
        Tile {passable : false,
              damage   : 0,
              image    : '!'}
    }

    pub fn new_full(new_passable : bool, new_damage : i32, new_image : char) -> Tile
    {
        Tile {passable : new_passable,
              damage   : new_damage,
              image    : new_image}
    }

    pub fn get_passable(&self) -> &bool
    {
        &self.passable
    }

    pub fn set_passable(&mut self, new_passable : bool)
    {
        self.passable = new_passable;
    }

    pub fn get_damage(&self) -> &i32
    {
        &self.damage
    }

    pub fn set_damage(&mut self, new_damage : i32)
    {
        self.damage = new_damage
    }

    pub fn get_image(&self) -> &char
    {
        &self.image
    }

    pub fn set_image(&mut self, new_image : char)
    {
        self.image = new_image;
    }
    
}

pub fn build_tile_database() -> [Tile; MAX_TILES]
{
    let default_tile = Tile {passable : true,
                             damage   : 0,
                             image    : '~'};

    let mut tile_database = [default_tile; MAX_TILES];
    tile_database[0].image = '.';

    tile_database[1].image = 't';
    tile_database[1].passable = false;

    
    tile_database  // Return the tile database.
}

pub fn get_tile_by_image(image : &char, tile_database: &[Tile; MAX_TILES]) -> Option<Tile>
{
    for i in 0 .. MAX_TILES
    {
        if tile_database[i].image == *image
        {
            return Some(tile_database[i]);
        }
    }
    
    None
}

pub fn print_tile_database(tile_database: &[Tile; MAX_TILES])
{
    for i in 0 .. MAX_TILES
    {
        println!("{}", tile_database[i].image);
    }
}
