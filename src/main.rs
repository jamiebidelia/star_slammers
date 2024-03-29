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
mod chargen;
mod menuaction;
mod tile;
mod tile_map;
mod console;
mod input;
mod menu;
mod mode;
mod inventory_screen;
mod rng;
mod skill;
mod textwriter;
mod attributeslider;
mod title;


/// Starts the game
fn main()
{
    let game_window   = initialize_game();   
    let mut end_game  = false;

    // Start the game in Menu Mode (The Title Screen)
    let main_menu               = menu::create_main_menu();
    let mut menu_cursor : usize = 0;
    let mut game_mode           = mode::Mode::TitleScreen;
    enter_title_mode(&mut game_mode);

    // The default player will be replaced by either character creation
    // Or by loading a character file.  If we ever see the default player
    // In real gameplay, we should try to debug that.
    let mut player = creature::Creature::Default_Player();
    
    // Creatures on Map contains each creature that is in this area.
    // The player character is always index 0.
    let mut creatures_on_map: Vec<creature::Creature> = Vec::new();
    creatures_on_map.push(player);
    
    // The Console Buffer will hold the messages that we want to display.
    let mut console_buffer : Vec<String> = Vec::new();
    
    // Tile Database holds the definition of each tile we want to use.
    let tile_database = tile::build_tile_database();

    // The Tile Map holds the terrain data for each square on the map.
    let mut tile_map = tile_map::load_map("maps/test.map".to_string(),
                                          &tile_database);

    let mut game_camera = camera::Camera::new();
    camera::update_camera(&mut game_camera,
                          &game_window,
                          &creatures_on_map[0],
                          &tile_map);


    
    
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
            },
            mode::Mode::TitleScreen =>
            {
                title_iter(&game_window,
                           &mut game_mode,
                           &mut menu_cursor,
                           &main_menu);
            },
            mode::Mode::CharGen =>
            {
                chargen_iter(&game_window,
                             &mut game_mode,
                             &mut creatures_on_map[0]);
            }
            

            mode::Mode::Quit =>
            {
                end_game = true;
            }

       
        } // End Match game_mode.

        
        std::thread::sleep(std::time::Duration::from_millis(100));
    } // End Game Loop.
    
    shut_down_game();
} // End Main.

fn enter_title_mode(game_mode    : &mut mode::Mode)
{
    *game_mode     = mode::Mode::TitleScreen;
}


/// In Adventure Mode, each iteration is a step in the game loop.
/// We draw the screen, take an action, and update the game camera.
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
                        &console_buffer);   // Draw the Screen.


    // Listen for a key and turn it into an action.
    let game_action = input::process_keyboard(&game_window); 
    action::do_action(&game_action,
                      game_window,
                      tile_map,
                      game_mode,
                      &mut creatures_on_map[0],
                      end_game,
                      console_buffer);      // Process the game action.

} // End adventure_iter.

/// In Inventory Mode, each iteration handles player input for changing gear,
/// checking stats, etc.
fn inventory_iter(game_window        : &pancurses::Window,
                  mut player         : &mut creature::Creature,
                  mut console_buffer : &mut Vec<String>)
{
    inventory_screen::draw_screen(&game_window,
                                  &mut player,
                                  &mut console_buffer);

    // Listen for a key and turn it into an action.
    let inventory_action = inventory_screen::process_keyboard(&game_window);
    
} // End inventory_iter.



/// In Title Mode, each iteration draws the title image and menus.  It also
/// handles player input for menu navigation.
fn title_iter(game_window  : &pancurses::Window,
              game_mode    : &mut mode::Mode,
              menu_cursor  : &mut usize,
              menu         : &Vec<menu::MenuItem>)
{
    title::draw_title(game_window);
    
    title::draw_menu(game_window, &menu_cursor);
    title::draw_dedication(game_window);
    
    // Listen for a key and turn it into an action.
    let menu_action = title::process_keyboard(&game_window);
    
    menuaction::do_action(game_mode,
                          &menu_action,
                          menu_cursor,
                          menu);
}


/// In CharGen Mode, we get the name and attributes of the character.
/// Mode automatically transitions to Adventure mode afterward.
/// TODO:  be able to back out of this to the main menu.
fn chargen_iter(game_window : &pancurses::Window,
                game_mode   : &mut mode::Mode,
                player      : &mut creature::Creature)
{
    *player    = chargen::do_chargen(game_window);
    *game_mode = mode::Mode::Adventure;
}


/// Gets PanCurses up and running and accepts keyboard input.
fn initialize_game() -> pancurses::Window
{
    let game_window = pancurses::initscr(); // Create a new window.
    pancurses::cbreak();                    // Allow one-character-at-a-time.
    pancurses::noecho();                    // Suppress echoing of characters.
    game_window.keypad(true);               // Set Keypad mode.
    game_window.nodelay(false);             // Set delay mode.
    pancurses::curs_set(0);                 // Disable cursor blinking.
    game_window                             // Return the window we initialized.     
}

/// Ends the Pancurses Window.
fn shut_down_game()
{
    pancurses::use_default_colors();        // Reset the terminal colors.
    pancurses::endwin();                    // End the window when we are done.
}

/// Shuts the game down properly before causing an assertion.
fn blow_up(err : String)
{
    shut_down_game();                          // Shut down the pancurses window.
    panic!(err);                               // We will now crash the game.
}
