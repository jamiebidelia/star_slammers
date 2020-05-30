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
#![allow(clippy::suspicious_else_formatting)]

extern crate pancurses;
use crate::creature;
use crate::direction;
use crate::tile_map;
use crate::console;
use crate::mode;

pub enum Action
{
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Pass,
    Invalid,
    Resize,
    Inventory,
    QuitMode,
    EndGame
}

pub fn do_action(game_action    : &Action,
                 game_window    : &pancurses::Window,
                 game_map       : &tile_map::TileMap,
                 game_mode      : &mut mode::Mode,
                 actor          : &mut creature::Creature,
                 end_game       : &mut bool,
                 mut console_buffer : &mut Vec<String>)
{
    
    match game_action
    {
        Action::MoveUp    => make_move(actor, direction::Direction::North, &game_map, &mut console_buffer),
        Action::MoveDown  => make_move(actor, direction::Direction::South, &game_map, &mut console_buffer),
        Action::MoveLeft  => make_move(actor, direction::Direction::East,  &game_map, &mut console_buffer),
        Action::MoveRight => make_move(actor, direction::Direction::West,  &game_map, &mut console_buffer),
        Action::Pass      =>
        {
            let message = format!("{}:  Pass.", actor.get_name());
            console::post_to_console(message,
                                     &mut console_buffer)
        },
        Action::Invalid   => (),
        Action::Resize    => {game_window.clear();},
        Action::Inventory => *game_mode = mode::Mode::Inventory,
        Action::QuitMode  => *game_mode = mode::Mode::TitleScreen, 
        Action::EndGame   => *end_game = true,
    }
}

fn make_move(actor          : &mut creature::Creature,
             direction      : direction::Direction,
             game_map       : &tile_map::TileMap,
             mut console_buffer : &mut Vec<String>)
{

    // Grab the x index of the actor.
    let actor_x_index = actor.get_x_pos();
    let actor_x_index = *actor_x_index as usize;

    // Grab the y index of the actor.
    let actor_y_index = actor.get_y_pos();
    let actor_y_index = *actor_y_index as usize;

    let map = game_map.get_map();

    
    match direction
    {
        direction::Direction::North =>
            if (actor_y_index < (*game_map.get_y_size() - 1) as usize) && // Remember that if y_size is 10, the max tile is tile 9.
               (*map[actor_x_index][actor_y_index + 1].get_passable())

            {
                actor.set_y_pos(actor.get_y_pos() + 1);
            }
           else
           {
               let message = format!("{}:  Cannot Go North!", actor.get_name());
               console::post_to_console(message, &mut console_buffer);
           },
        direction::Direction::South =>
            if (actor_y_index > 0) &&
               (*map[actor_x_index][actor_y_index - 1].get_passable())
            {
                actor.set_y_pos(actor.get_y_pos() - 1);
            }
            else
            {
                let message = format!("{}:  Cannot Go South!", actor.get_name());
                console::post_to_console(message, &mut console_buffer);
            },
        direction::Direction::East =>
            if (actor_x_index > 0) &&
               (*map[actor_x_index - 1][actor_y_index].get_passable())
            {
                actor.set_x_pos(actor.get_x_pos() - 1);
            }
            else
            {  
                let message = format!("{}:  Cannot Go East!", actor.get_name());
                console::post_to_console(message, &mut console_buffer);
            },
        direction::Direction::West =>
            if (actor_x_index < (*game_map.get_x_size() - 1) as usize) && // Remember that if x_size is 10, the max tile is tile 9.
               (*map[actor_x_index + 1][actor_y_index].get_passable())
            {
                actor.set_x_pos(actor.get_x_pos() + 1);
            }
            else
            {
                let message = format!("{}:  Cannot Go West!", actor.get_name());
                console::post_to_console(message, &mut console_buffer);
            }
    } // End match direction.
}
