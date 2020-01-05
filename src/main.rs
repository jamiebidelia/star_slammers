///TOP LEVEL DOC COMMENT.
/*
colors are:

COLOR_BLACK
COLOR_RED
COLOR_GREEN
COLOR_YELLOW
COLOR_BLUE
COLOR_MAGENTA
COLOR_CYAN
COLOR_WHITE
*/





extern crate pancurses;


mod item;
mod creature;
mod action;
mod direction;
mod camera;
mod tile;
mod tile_map;
mod console;
mod input;
mod mode;
mod inventory_screen;


fn main()
{
    let mut game_mode        = mode::Mode::Adventure;
    let game_window          = initialize_game();   
    let mut end_game         = false;

    let mut player = creature::Creature::new();
    player.set_name("Avatar Steve".to_string());
    player.set_player_control(true);
    player.set_image('H');
    player.set_x_pos(0);
    player.set_y_pos(0);

    // Creatures on Map contains each creature that is in this area.
    let mut creatures_on_map: Vec<creature::Creature> = Vec::new();
    creatures_on_map.push(player);
    
    // The Console Buffer will hold the messages that we want to display.
    let mut console_buffer : Vec<String> = Vec::new();

    
    // Tile Database holds the definition of each tile we want to use.
    let tile_database = tile::build_tile_database();

    // The Tile Map holds the terrain data for each square on the map.
    let mut tile_map = tile_map::load_map("maps/test.map".to_string(), &tile_database);

    let mut game_camera = camera::Camera::new();
    camera::update_camera(&mut game_camera, &game_window, &creatures_on_map[0], &tile_map);


    
    
    // Game Loop:  Get Input, Process Input, Process All Events.
    while !end_game
    {
        match game_mode
        {
            mode::Mode::Adventure =>
            {
                adventure_iter(&game_window,
                               &mut game_camera,
                               &mut tile_map,
                               &mut creatures_on_map,
                               &mut console_buffer,
                               &mut game_mode,
                               &mut end_game);

            },
            
            mode::Mode::Inventory =>
            {
                inventory_iter(&game_window,
                               &mut creatures_on_map[0],
                               &mut console_buffer);
            }


        } // End Match game_mode.

        
        std::thread::sleep(std::time::Duration::from_millis(100));
    } // End Game Loop.

    shut_down_game();
} // End Main.

fn adventure_iter(game_window       : &pancurses::Window,
                  game_camera       : &mut camera::Camera,
                  tile_map          : &mut tile_map::TileMap,
                  creatures_on_map  : &mut Vec<creature::Creature>,
                  console_buffer    : &mut Vec<String>,
                  game_mode         : &mut mode::Mode,
                  end_game          : &mut bool)
{
    camera::draw_screen(&game_window,
                        &game_camera,
                        &tile_map,
                        &creatures_on_map,
                        console_buffer);	// Draw the Screen.
    
    let game_action = input::process_keyboard(&game_window); // Listen for a key and turn it into an action.
    action::do_action(&game_action,
                      game_window,
                      tile_map,
                      game_mode,
                      &mut creatures_on_map[0],
                      end_game,
                      console_buffer);   // Process the game action.
    
    camera::update_camera(game_camera, game_window, &mut creatures_on_map[0], tile_map);
} // End adventure_iter.


fn inventory_iter(game_window        : &pancurses::Window,
                  mut player         : &mut creature::Creature,
                  mut console_buffer : &mut Vec<String>)
{
    inventory_screen::draw_screen(&game_window,
                                  &mut player,
                                  &mut console_buffer);
    let inventory_action = inventory_screen::process_keyboard(&game_window); // Listen for a key and turn it into an action.
    
} // End inventory_iter.

fn initialize_game() -> pancurses::Window
{
    let game_window = pancurses::initscr(); // Create a new window.
    pancurses::cbreak();		    // Allow one-character-at-a-time.
    pancurses::noecho();		    // Suppress echoing of characters.
    game_window.keypad(true);		    // Set Keypad mode.
    game_window.nodelay(false);		    // Set delay mode.
    pancurses::curs_set(0);                 // Disable cursor blinking.
    game_window				    // Return the window we initialized.     
}

fn shut_down_game()
{
    pancurses::use_default_colors();        // Make sure the terminal colors are reset.
    pancurses::endwin();	            // End the window when we are done.
}

fn blow_up()
{
    shut_down_game();                       // Shut down the pancurses window.
    assert!(false);                         // We will now crash the game.
}
