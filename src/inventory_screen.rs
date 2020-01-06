
extern crate pancurses;
use crate::action;
use crate::creature;

pub fn process_keyboard(game_window : &pancurses::Window) -> action::Action
{
    let mut game_action = action::Action::Invalid;

    match game_window.getch()
    {
        Some(pancurses::Input::KeyResize) =>
        {
        }
        Some(pancurses::Input::KeyUp) =>
        {
        }
        Some(pancurses::Input::KeyDown) =>
        {
        }
        Some(pancurses::Input::KeyLeft) =>
        {
        }
        Some(pancurses::Input::KeyRight) =>
        {
        }
        Some(pancurses::Input::Character('q')) =>
        {
            game_action = action::Action::QuitMode;
        }
        Some(pancurses::Input::Character(_)) =>
        {
            game_action = action::Action::EndGame;
        }
        Some(_) =>
        {
        }
        None => () // Do nothing.
    }

    game_action
}


fn get_map_end_x(game_window : &pancurses::Window) -> i32
{
    const MAP_X_PROPORTION : f32 = 0.60;
    
    let start_x = game_window.get_beg_x();
    let end_x   = game_window.get_max_x();

    let map_start_x = start_x;
    
    // Compute the ending point for map X, and make it an Integer.
    let map_end_x = (end_x - map_start_x) as f32;
    let map_end_x = MAP_X_PROPORTION * map_end_x;

    map_end_x.floor() as i32
}

fn get_map_end_y(game_window : &pancurses::Window) -> i32
{
    const MAP_Y_PROPORTION : f32 = 0.75;
    
    let start_y = game_window.get_beg_y();
    let end_y   = game_window.get_max_y();

    let map_start_y = start_y;

    // Compute the ending point for map Y, and make it an Integer.
    let map_end_y = (end_y - map_start_y) as f32;
    let map_end_y = MAP_Y_PROPORTION * map_end_y;

    map_end_y.floor() as i32
}

fn get_console_start_x(game_window : &pancurses::Window) -> i32
{
    game_window.get_beg_x() + 1
}

fn get_console_end_x(game_window : &pancurses::Window) -> i32
{
    get_map_end_x(game_window)
}

fn get_console_start_y(game_window : &pancurses::Window) -> i32
{
    get_map_end_y(game_window)
}

fn get_console_end_y(game_window : &pancurses::Window) -> i32
{
    game_window.get_max_y() - 2
}


// TODO: Write the routine to draw the screen when we are in the inventory.
pub fn draw_screen(game_window    : &pancurses::Window,
                   player         : &mut creature::Creature,
                   console_buffer : &mut Vec<String>)
{
    
    // The max values it gives are not printable.  So we need to
    // Subtract 1 from each to reach our last index.  Be mindful of that.
    let start_x = game_window.get_beg_x();
    let end_x   = game_window.get_max_x();

    let start_y = game_window.get_beg_y();
    let end_y   = game_window.get_max_y();

    let map_start_x = start_x;
    let map_start_y = start_y;

    if pancurses::can_change_color()
    {
        pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
        game_window.color_set(1);
    }
    
    game_window.erase();
    game_window.refresh();
           
}
