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

//! This module contains the test code for the camera module.

#[cfg(test)]
mod tests
{
    //update_camera tests.
    //   Arguments to validate:
    //      game_camera: &mut Camera
    //      game_window: &pancurses::Window
    //      actor:       &creature::Creature
    //      game_map:    &tile_map::TileMap

    // Question:  Why are we defining a reference to a camera here instead of using self?
    // Remove this?

    // New Requirement:  Build validation functions that can be called for each
    // Camera method so can test interfaces without messy code duplication.

    
    // Let's validate that the actor is in a position that is on the freaking map.
    // Let's also validate that the actor is drawable.
    #[test]
    #[should_panic]
    fn update_camera_actor_off_map_x()
    {

        let mut game_camera = crate::camera::Camera::new();

        let game_window     = crate::initialize_game();
        
        let mut player      = crate::creature::Creature::new();
        player.set_x_pos(10000); // This should be off the map.

        // Tile Database holds the definition of each tile we want to use.
        let tile_database   = crate::tile::build_tile_database();

        // The Tile Map holds the terrain data for each square on the map.
        let mut tile_map    = crate::tile_map::load_map("maps/test.map".to_string(),
                                                      &tile_database);
        
        // This call should panic, as the character is not within the bounds of the game!
        crate::camera::update_camera(&mut game_camera, &game_window, &player, &tile_map);
    }

    #[test]
    #[should_panic]
    fn update_camera_actor_off_map_y()
    {
        let mut game_camera = crate::camera::Camera::new();

        let game_window     = crate::initialize_game();
        
        let mut player      = crate::creature::Creature::new();
        player.set_y_pos(10000); // This should be off the map.
        let tile_database   = crate::tile::build_tile_database();
        let mut tile_map    = crate::tile_map::load_map("maps/test.map".to_string(),
                                                        &tile_database);
        crate::shut_down_game();
        
        // This call should panic, as the character is not within the bounds of the game!
        crate::camera::update_camera(&mut game_camera, &game_window, &player, &tile_map);
    }

    #[test]
    #[should_panic]
    fn update_camera_tile_map_zero_size_x()
    {
        let mut game_camera = crate::camera::Camera::new();
        
        let game_window     = crate::initialize_game();
        
        let mut player      = crate::creature::Creature::new();
        let tile_database   = crate::tile::build_tile_database();
        let mut tile_map    = crate::tile_map::load_map("maps/test.map".to_string(),
                                                        &tile_database);
        
        tile_map.set_x_size(0); //Gives the map a 0-dimensional x; should NEVER happen.
        
        // This call should panic, as the character is not within the bounds of the game!
        crate::camera::update_camera(&mut game_camera, &game_window, &player, &tile_map);
    }

    #[test]
    #[should_panic]
    fn update_camera_tile_map_zero_size_y()
    {
        let mut game_camera = crate::camera::Camera::new();
        
        let game_window     = crate::initialize_game();
        
        let mut player      = crate::creature::Creature::new();
        let tile_database   = crate::tile::build_tile_database();
        let mut tile_map    = crate::tile_map::load_map("maps/test.map".to_string(),
                                                        &tile_database);
        
        tile_map.set_y_size(0); //Gives the map a 0-dimensional x; should NEVER happen.
        
        // This call should panic, as the character is not within the bounds of the game!
        crate::camera::update_camera(&mut game_camera, &game_window, &player, &tile_map);
    }
    
    //draw_screen tests
    
    //draw_console tests
}
